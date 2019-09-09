use crate::dot::Dot;
use crate::dfg::Alias;
use crate::cfg::ControlFlowGraph;
use crate::dfg::DataFlowGraph;
use crate::core::{
    LookupInputType,
    DataLink,
    DataLinkLabel,
    Dictionary,
    Member,
    Variable,
    Action,
};
use crate::dfg::utils;
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
        let mut ret = HashSet::new();
        ret
    } 

    fn find_internal_links(&mut self) -> HashSet<DataLink> {
        let mut links = HashSet::new();
        let walkers = self.dict.lookup_functions(LookupInputType::ContractId(self.entry_id));
        if walkers.is_empty() {
            let cfg = ControlFlowGraph::new(self.dict, self.entry_id);
            let alias = Alias::new(&cfg);
            let mut dfg = DataFlowGraph::new(cfg, alias);
            links.extend(dfg.find_links());
            self.dfgs.insert(self.entry_id, dfg);
        } else {
            for walker in walkers {
                let cfg = ControlFlowGraph::new(self.dict, walker.node.id);
                let alias = Alias::new(&cfg);
                let mut dfg = DataFlowGraph::new(cfg, alias);
                links.extend(dfg.find_links());
                self.dfgs.insert(walker.node.id, dfg);
            }
        }
        links
    }

    fn find_links(&mut self) {
        let internal_links = self.find_internal_links();
        let external_links = self.find_external_links();
        self.links.extend(internal_links);
        self.links.extend(external_links);
        // Find all sub networks 
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
