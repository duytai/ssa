use crate::dot::Dot;
use crate::cfg::ControlFlowGraph;
use crate::dfg::DataFlowGraph;
use crate::core::{
    DataLink,
    DataLinkLabel,
    Dictionary,
    Member,
    Variable,
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
    entry_id: u32,
}

impl<'a> Network<'a> {
    pub fn new(dict: &'a Dictionary, entry_id: u32) -> Self {
        let mut network = Network {
            dict,
            links: HashSet::new(),
            dfgs: HashMap::new(),
            dot: Dot::new(),
            entry_id,
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

    pub fn get_entry_id(&self) -> u32 {
        self.entry_id
    }

    fn find_external_links(&mut self) -> HashSet<DataLink> {
        let mut links = HashSet::new(); 
        let function_calls = self.dict.lookup_function_calls(self.entry_id);
        for walker in function_calls.iter() {
            let walkers = walker.direct_childs(|_| true);
            let source = walker.node.source;
            let fc_id = walker.node.id;
            let reference = walker.node
                .attributes["type"]
                .as_str()
                .and_then(|return_type| {
                    if return_type.starts_with("contract") {
                        // let contract_id = self.dict.lookup_contract_by_name(&return_type[9..]);
                        None
                    } else {
                        walkers[0].node.attributes["referencedDeclaration"]
                            .as_u32()
                            .and_then(|reference| match self.dict.lookup(reference) {
                                Some(walker) => match walker.node.name {
                                    "EventDefinition" => None,
                                    _ => Some(reference),
                                },
                                None => None,
                            })
                    }
                });
            match reference {
                // User defined functions
                // Connect invoked_parameters to defined_parameters 
                // Connect function_call to return statement
                Some(reference) => {
                    for walker in self.dict.lookup_returns(reference) {
                        let members = vec![Member::Reference(walker.node.id)];
                        let variable = Variable::new(members, source.to_string());
                        let label = DataLinkLabel::InFrom(fc_id);
                        let link = DataLink::new_with_label(fc_id, walker.node.id, variable, label);
                        links.insert(link);
                    }
                    let defined_parameters = self.dict.lookup_parameters(reference);
                    let mut invoked_parameters = self.dict.lookup_parameters(fc_id);
                    if invoked_parameters.len() < defined_parameters.len() {
                        invoked_parameters.insert(0, &walkers[0]);
                    }
                    for i in 0..invoked_parameters.len() {
                        let defined_parameter = defined_parameters[i];
                        let invoked_parameter = invoked_parameters[i];
                        let members = vec![Member::Reference(invoked_parameter.node.id)];
                        let variable = Variable::new(members, defined_parameter.node.source.to_string());
                        let label = DataLinkLabel::OutTo(fc_id);
                        let link = DataLink::new_with_label(defined_parameter.node.id, invoked_parameter.node.id, variable, label);
                        links.insert(link);
                    }
                },
                // Emit event
                // Global functions
                // Connect function_calls to parameters
                None => {
                    let invoked_parameters = self.dict.lookup_parameters(fc_id);
                    for invoked_parameter in invoked_parameters {
                        let members = vec![Member::Reference(invoked_parameter.node.id)];
                        let variable = Variable::new(members, invoked_parameter.node.source.to_string());
                        let label = DataLinkLabel::BuiltIn;
                        let link = DataLink::new_with_label(fc_id, invoked_parameter.node.id, variable, label);
                        links.insert(link);
                    }
                },
            };
            // Connect to object which call function
            let members = vec![Member::Reference(walkers[0].node.id)];
            let source = walkers[0].node.source;
            let variable = Variable::new(members, source.to_string());
            let label = DataLinkLabel::Executor;
            let link = DataLink::new_with_label(fc_id, walkers[0].node.id, variable, label);
            links.insert(link);
        }
        links
    } 

    fn find_internal_links(&mut self) -> HashSet<DataLink> {
        let mut links = HashSet::new();
        for walker in self.dict.lookup_functions_by_contract_id(self.entry_id) {
            let cfg = ControlFlowGraph::new(self.dict, walker.node.id);
            let mut dfg = DataFlowGraph::new(cfg);
            links.extend(dfg.find_links());
            self.dfgs.insert(walker.node.id, dfg);
        }
        links
    }

    fn find_links(&mut self) {
        let internal_links = self.find_internal_links();
        let external_links = self.find_external_links();
        self.links.extend(internal_links);
        self.links.extend(external_links);
    }

    /// Find all paths
    /// keep call_stack for each path to know correct exit point
    fn find_paths(&'a self, start_at: u32, mut visited: HashSet<(Option<u32>, u32)>, paths: &mut Vec<Vec<&'a DataLink>>, call_stack: Vec<u32>) {
        let mut targets = vec![];
        for link in self.links.iter() {
            if link.get_from() == start_at {
                match link.get_label() {
                    DataLinkLabel::InFrom(fc_id) => {
                        let mut new_call_stack = call_stack.clone();
                        new_call_stack.push(*fc_id);
                        targets.push((link, new_call_stack));
                    },
                    DataLinkLabel::OutTo(fc_id) => {
                        if let Some(id) = call_stack.last() {
                            if id == fc_id {
                                let mut new_call_stack = call_stack.clone();
                                new_call_stack.pop();
                                targets.push((link, new_call_stack));
                            }
                        }
                    },
                    DataLinkLabel::Internal | DataLinkLabel::BuiltIn | DataLinkLabel::Executor => {
                        targets.push((link, call_stack.clone()));
                    },
                }
            }
        }
        let last_stack_item: Option<u32> = call_stack.last().map(|x| *x);
        if !visited.contains(&(last_stack_item, start_at)) && !targets.is_empty() {
            let prev_paths = paths.clone();
            paths.clear();
            for path in prev_paths {
                let last_link = path.last().unwrap();
                if last_link.get_to() == start_at {
                    for (link, _) in targets.iter() {
                        let mut new_path = path.clone();
                        new_path.push(link);
                        paths.push(new_path);
                    }
                } else {
                    paths.push(path);
                }
            }
            for (link, call_stack) in targets {
                let last_stack_item: Option<u32> = call_stack.last().map(|x| *x);
                visited.insert((last_stack_item, start_at));
                self.find_paths(link.get_to(), visited.clone(), paths, call_stack);
            }
        }
    }

    /// Traverse around network
    /// Stop at visited nodes or no more link
    pub fn traverse(&self, start_at: u32) -> Vec<Vec<&DataLink>> {
        let mut paths = vec![];
        let mut targets: Vec<(&DataLink, Vec<u32>)> = vec![];
        // (X, Y) where
        // X is call_stack id
        // Y is node id
        let mut visited: HashSet<(Option<u32>, u32)> = HashSet::new(); 
        for link in self.links.iter() {
            if link.get_from() == start_at {
                paths.push(vec![link]);
                match link.get_label() {
                    DataLinkLabel::InFrom(fc_id) => {
                        targets.push((link, vec![*fc_id]));
                    },
                    DataLinkLabel::Internal | DataLinkLabel::BuiltIn | DataLinkLabel::Executor => {
                        targets.push((link, vec![]));
                    },
                    DataLinkLabel::OutTo(_) => {},
                } 
            }
        }
        for (link, call_stack) in targets {
            let last_stack_item: Option<u32> = call_stack.last().map(|x| *x).clone();
            visited.insert((last_stack_item, start_at));
            self.find_paths(link.get_to(), visited.clone(), &mut paths, call_stack);
        }
        paths
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
