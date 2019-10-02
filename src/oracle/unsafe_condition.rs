use crate::dfg::Network;
use crate::core::Action;
use crate::core::Member;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct UnsafeSendingCondition {
    block_timestamps: HashSet<(u32, u32)>,
    block_numbers: HashSet<(u32, u32)>,
}

impl UnsafeSendingCondition {
    pub fn new(network: &Network) -> Self {
        let mut unsafe_sending_condition = UnsafeSendingCondition {
            block_timestamps: HashSet::new(),
            block_numbers: HashSet::new(),
        };
        unsafe_sending_condition.update(network);
        unsafe_sending_condition
    }

    fn update(&mut self, network: &Network) {
        let mut all_actions = HashMap::new();
        let mut all_edges = HashSet::new();
        let mut execution_paths = vec![];
        let dict = network.get_dict();

        for (_, dfg) in network.get_dfgs().iter() {
            let cfg = dfg.get_cfg();
            all_actions.extend(dfg.get_new_actions());
            all_edges.extend(cfg.get_edges());
            execution_paths.extend(cfg.get_execution_paths());
        }

        let get_outdegree = |from: u32| -> u32 {
            let mut degree = 0;
            for edge in all_edges.iter() {
                if edge.get_from() == from {
                    degree += 1;
                }
            }
            degree
        };

        let get_variables = |id: u32| {
            let mut variables = HashSet::new();
            if let Some(actions) = all_actions.get(&id) {
                for action in actions.iter() {
                    match action {
                        Action::Use(variable, _) => {
                            variables.insert(variable.clone());
                        },
                        Action::Kill(variable, _) => {
                            variables.insert(variable.clone());
                        },
                    }
                }
            }
            variables
        };

        let mut possible_vul_vertices: HashSet<u32> = HashSet::new();
        for execution_path in execution_paths {
            let mut idx = execution_path.len() - 1;
            while idx > 0 {
                let vertex_id = execution_path[idx];
                if let Some(walker) = dict.walker_at(vertex_id) {
                    if walker.node.name == "FunctionCall" {
                        for variable in get_variables(vertex_id) {
                            if let Some(last_member) = variable.get_members().last() {
                                let sending_methods = vec![
                                    Member::Global(String::from("send")),
                                    Member::Global(String::from("transfer")),
                                    Member::Global(String::from("call")),
                                    Member::Global(String::from("callcode")),
                                    Member::Global(String::from("delegatecall")),
                                    Member::Global(String::from("selfdestruct")),
                                    Member::Global(String::from("suicide")),
                                ];
                                if sending_methods.contains(last_member) {
                                    possible_vul_vertices.insert(vertex_id);
                                    for i in 0..idx {
                                        let vertex_id = execution_path[i];
                                        let outdegree = get_outdegree(vertex_id);
                                        if outdegree >= 2 {
                                            possible_vul_vertices.insert(vertex_id);
                                        }
                                    }
                                    idx = 1;
                                }
                            }
                        }
                    }
                }
                idx -= 1;
            }
        }

        for vertex_id in possible_vul_vertices {
            for variable in get_variables(vertex_id) {
                let source = variable.get_source();
                match (source.starts_with("block.number"), source.starts_with("block.timestamp") || source.starts_with("now")) {
                    (true, _) => {
                        self.block_numbers.insert((vertex_id, vertex_id));
                    },
                    (_, true) => {
                        self.block_timestamps.insert((vertex_id, vertex_id));
                    },
                    _ => {
                        let source = (variable.clone(), vertex_id);
                        for dependent_path in network.traverse(source) {
                            if dependent_path.len() > 1 {
                                let (variable, dependent_id) = dependent_path.last().unwrap();
                                let source = variable.get_source();
                                if source == "block.number" {
                                    self.block_numbers.insert((vertex_id, *dependent_id));
                                }
                                if source == "block.timestamp" || source == "now" {
                                    self.block_timestamps.insert((vertex_id, *dependent_id));
                                }
                            }
                        }
                    },
                }
            }
        }
    }

    pub fn get_block_numbers(&self) -> &HashSet<(u32, u32)> {
        &self.block_numbers
    }

    pub fn get_block_timestamps(&self) -> &HashSet<(u32, u32)> {
        &self.block_timestamps
    }
}
