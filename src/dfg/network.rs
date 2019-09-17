use crate::dot::Dot;
use crate::cfg::ControlFlowGraph;
use crate::dfg::DataFlowGraph;
use crate::core::{
    DataLinkLabel,
    DataLink,
    Dictionary,
    SmartContractQuery,
    Action,
    VariableComparison,
};
use std::collections::{
    HashMap,
    HashSet,
};

pub struct Network<'a> {
    dict: &'a Dictionary<'a>,
    links: HashSet<DataLink>,
    dfgs: HashMap<u32, DataFlowGraph<'a>>,
    dot: Dot,
    contract_id: u32,
}

impl<'a> Network<'a> {
    pub fn new(dict: &'a Dictionary, contract_id: u32) -> Self {
        let mut network = Network {
            dict,
            links: HashSet::new(),
            dfgs: HashMap::new(),
            dot: Dot::new(),
            contract_id,
        };
        network.find_links();
        network
    }

    pub fn get_links(&self) -> &HashSet<DataLink> {
        &self.links
    }

    pub fn get_dfgs(&self) -> &HashMap<u32, DataFlowGraph> {
        &self.dfgs
    }

    pub fn get_dict(&self) -> &Dictionary {
        &self.dict
    }

    pub fn get_contract_id(&self) -> u32 {
        self.contract_id
    }

    fn find_assignment_links(&mut self) -> HashSet<DataLink> {
        let mut assignment_links = HashSet::new();
        let mut all_actions = HashMap::new();
        for (_, dfg) in self.dfgs.iter() {
            all_actions.extend(dfg.get_new_actions());
        }
        for (vertex_id, actions) in all_actions {
            let mut kill_variables = HashSet::new();
            let mut use_variables = HashSet::new();
            for action in actions {
                match action {
                    Action::Use(variable, _) => {
                        use_variables.insert(variable.clone());
                    },
                    Action::Kill(variable, _) => {
                        kill_variables.insert(variable.clone());
                    },
                }
            }
            if kill_variables.len() == 1 {
                for kill_variable in kill_variables.iter() {
                    for use_variable in use_variables.iter() {
                        if kill_variable.get_kind() == use_variable.get_kind() {
                            let data_link = DataLink::new(
                                (kill_variable.clone(), *vertex_id),
                                (use_variable.clone(), *vertex_id),
                                DataLinkLabel::SameType,
                            );
                            assignment_links.insert(data_link);
                        }
                    }
                }
            }
            if kill_variables.len() > 1 {
                for kill_variable in kill_variables.iter() {
                    for use_variable in use_variables.iter() {
                        if kill_variable.equal_property(use_variable) {
                            let data_link = DataLink::new(
                                (kill_variable.clone(), *vertex_id),
                                (use_variable.clone(), *vertex_id),
                                DataLinkLabel::SameType,
                            );
                            assignment_links.insert(data_link);
                        }
                    }
                }
            }
        }
        assignment_links
    }

    fn find_index_links(&mut self) -> HashSet<DataLink> {
        let mut index_links = HashSet::new();
        let mut all_actions = HashMap::new();
        let mut all_indexes = HashMap::new();
        for (_, dfg) in self.dfgs.iter() {
            let cfg = dfg.get_cfg();
            all_actions.extend(dfg.get_new_actions());
            all_indexes.extend(cfg.get_indexes().clone());
        }
        let get_variables = |index_id: u32| {
            let mut variables = HashSet::new();
            if let Some(actions) = all_actions.get(&index_id) {
                for action in actions.iter() {
                    if let Action::Use(variable, _) = action {
                        variables.insert(variable.clone());
                    }
                }
            }
            variables
        };
        for (index_id, params) in all_indexes {
            let index_variables = get_variables(index_id);
            for index_param_id in &params[2..] {
                let param_variables = get_variables(*index_param_id);
                for index_variable in index_variables.iter() {
                    for param_variable in param_variables.iter() {
                        let data_link = DataLink::new(
                            (index_variable.clone(), index_id),
                            (param_variable.clone(), *index_param_id),
                            DataLinkLabel::SwitchType,
                        );
                        index_links.insert(data_link);
                    }
                }
            } 
            {
                let param_variables = get_variables(params[1]);
                for index_variable in index_variables.iter() {
                    for param_variable in param_variables.iter() {
                        if param_variable.equal_property(index_variable) {
                            let data_link = DataLink::new(
                                (index_variable.clone(), index_id),
                                (param_variable.clone(), params[1]),
                                DataLinkLabel::SameType,
                            );
                            index_links.insert(data_link);
                        }
                    }
                }
            }
            self.dict.walker_at(params[0]).map(|walker| {
                if walker.node.name != "IndexAccess" {
                    let instance_variables = get_variables(walker.node.id);
                    for instance_variable in instance_variables.iter() {
                        for index_variable in instance_variables.iter() {
                            if index_variable.equal_property(instance_variable) {
                                let data_link = DataLink::new(
                                    (instance_variable.clone(), params[0]),
                                    (index_variable.clone(), index_id),
                                    DataLinkLabel::SameType,
                                );
                                index_links.insert(data_link);
                            }
                        }
                    }
                }
            });
        }
        index_links
    }

    fn find_fcall_links(&mut self) -> HashSet<DataLink>  {
        let mut fcall_links = HashSet::new();
        let mut all_actions = HashMap::new();
        let mut all_fcalls = HashMap::new();
        let mut all_returns = HashMap::new();
        for (_, dfg) in self.dfgs.iter() {
            let cfg = dfg.get_cfg();
            all_actions.extend(dfg.get_new_actions());
            all_fcalls.extend(cfg.get_fcalls().clone());
            all_returns.extend(cfg.get_returns().clone());
        }
        let get_variables = |index_id: u32| {
            let mut variables = HashSet::new();
            if let Some(actions) = all_actions.get(&index_id) {
                for action in actions.iter() {
                    if let Action::Use(variable, _) = action {
                        variables.insert(variable.clone());
                    }
                }
            }
            variables
        };
        for (fcall_id, params) in all_fcalls {
            let fcall_variables = get_variables(fcall_id);
            self.dict.walker_at(fcall_id).map(|walker| {
                let walkers = walker.direct_childs(|_| true);
                let declaration = walkers[0].node.attributes["referencedDeclaration"].as_u32();
                let is_user_defined = declaration.and_then(|declaration| all_returns.get(&declaration)).is_some();
                match is_user_defined {
                    false => {
                        for param_id in (&params[2..]).iter() {
                            let param_variables = get_variables(*param_id);
                            for fcall_variable in fcall_variables.iter() {
                                for param_variable in param_variables.iter() {
                                    let data_link = DataLink::new(
                                        (fcall_variable.clone(), fcall_id),
                                        (param_variable.clone(), *param_id),
                                        DataLinkLabel::SwitchType,
                                    );
                                    fcall_links.insert(data_link);
                                }
                            }
                        }
                        {
                            let param_variables = get_variables(params[1]);
                            for fcall_variable in fcall_variables.iter() {
                                for param_variable in param_variables.iter() {
                                    if param_variable.equal_property(fcall_variable) {
                                        let data_link = DataLink::new(
                                            (fcall_variable.clone(), fcall_id),
                                            (param_variable.clone(), params[1]),
                                            DataLinkLabel::SameType,
                                        );
                                        fcall_links.insert(data_link);
                                    }
                                }
                            }
                        }
                        self.dict.walker_at(params[0]).map(|walker| {
                            if walker.node.name != "FunctionCall" {
                                let instance_variables = get_variables(walker.node.id);
                                for instance_variable in instance_variables.iter() {
                                    for fcall_variable in fcall_variables.iter() {
                                        if fcall_variable.equal_property(instance_variable) {
                                            let data_link = DataLink::new(
                                                (instance_variable.clone(), params[0]),
                                                (fcall_variable.clone(), fcall_id),
                                                DataLinkLabel::SameType,
                                            );
                                            fcall_links.insert(data_link);
                                        }
                                    }
                                }
                            }
                        });
                    },
                    true => {
                        // let declaration = declaration.unwrap();
                        // let returns = all_returns.get(&declaration).unwrap();
                        // for return_id in returns {
                            // let return_variables = get_variables(*return_id);
                            // for fcall_variable in fcall_variables.iter() {
                                // for return_variable in return_variables.iter() {
                                // }
                            // }
                        // }
                    }
                }
            });
        }
        fcall_links
    }

    fn find_external_links(&mut self) -> HashSet<DataLink> {
        let mut external_links = HashSet::new();
        // external_links.extend(self.find_assignment_links());
        external_links.extend(self.find_index_links());
        external_links.extend(self.find_fcall_links());
        external_links
    } 

    fn find_internal_links(&mut self) -> HashSet<DataLink> {
        let mut links = HashSet::new();
        let function_ids = self.dict.find_ids(SmartContractQuery::FunctionsByContractId(self.contract_id));
        for function_id in function_ids {
            let cfg = ControlFlowGraph::new(self.dict, self.contract_id, function_id);
            let mut dfg = DataFlowGraph::new(cfg);
            links.extend(dfg.find_links());
            self.dfgs.insert(function_id, dfg);
        }
        links
    }

    fn find_links(&mut self) {
        let internal_links = self.find_internal_links();
        let external_links = self.find_external_links();
        self.links.extend(internal_links);
        self.links.extend(external_links);
    }

    pub fn format(&mut self) -> String {
        self.dot.clear();
        for (_, dfg) in self.dfgs.iter() {
            self.dot.add_cfg(dfg.get_cfg());
        }
        self.dot.add_links(&self.links);
        self.dot.format()
    }
}
