use crate::dot::Dot;
use crate::cfg::ControlFlowGraph;
use crate::dfg::DataFlowGraph;
use crate::core::{
    DataLinkLabel,
    DataLink,
    Dictionary,
    SmartContractQuery,
    Action,
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
        for (_, dfg) in self.dfgs.iter() {
            all_actions.extend(dfg.get_new_actions());
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
        for index_id in self.dict.find_ids(SmartContractQuery::IndexesByContractId(self.contract_id)) {
            let index_variables = get_variables(index_id);
            for index_param_id in self.dict.find_ids(SmartContractQuery::IndexParamsByIndexAccess(index_id)) {
                let param_variables = get_variables(index_param_id);
                for index_variable in index_variables.iter() {
                    for param_variable in param_variables.iter() {
                        let data_link = DataLink::new(
                            (index_variable.clone(), index_id),
                            (param_variable.clone(), index_param_id),
                            DataLinkLabel::SwitchType,
                        );
                        index_links.insert(data_link);
                    }
                }
            }
        }
        index_links
    }

    fn find_external_links(&mut self) -> HashSet<DataLink> {
        let mut external_links = HashSet::new();
        // external_links.extend(self.find_assignment_links());
        external_links.extend(self.find_index_links());
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
