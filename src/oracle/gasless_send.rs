use std::collections::HashSet;
use crate::dfg::Network;
use crate::cfg::ControlFlowGraph;
use crate::core::{
    Action,
    DataLinkLabel,
    DataLink,
    Member,
    VariableComparison,
};

/// Detect gasless send <X>.send() / <X>.transfer()
/// - address <X> depends on parameters
///  + has no condition check on path to <X> 
/// - address <X> depends on msg.sender
///  + has no condition check on path to <X> 
pub struct GaslessSend<'a> {
    network: &'a Network<'a>,
}

impl<'a> GaslessSend <'a> {
    pub fn new(network: &'a Network<'a>) -> Self {
        GaslessSend { network }
    }

    fn get_states_ids(&self) -> Vec<u32> {
        let mut ret = vec![];
        let dict = self.network.get_dict();
        let entry_id = self.network.get_entry_id();
        for walker in dict.lookup_states(entry_id) {
            ret.push(walker.node.id);
        }
        ret
    }

    fn get_parameter_ids(&self) -> Vec<u32> {
        let mut ret = vec![];
        let dict = self.network.get_dict();
        for walker in dict.lookup_functions(self.network.get_entry_id()) {
            for walker in dict.lookup_parameters(walker.node.id) {
                ret.push(walker.node.id);
            }
        }
        ret
    }

    fn find_msg_sender(&self, link_to: u32) -> bool {
        let dfgs = self.network.get_dfgs();
        for (_, dfg) in dfgs {
            if let Some(actions) = dfg.get_new_actions().get(&link_to) {
                for action in actions {
                    if let Action::Use(var, _) = action {
                        let msg = Member::Global(String::from("msg"));
                        let sender = Member::Global(String::from("sender"));
                        if var.get_members() == &vec![sender, msg] {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn find_cfg_paths(&self, start_at: u32, cfg: &ControlFlowGraph, paths: &mut Vec<Vec<u32>>) {
        if paths.is_empty() {
            paths.push(vec![start_at]);
        }
        let mut childs = vec![];
        for edge in cfg.get_edges() {
            if edge.get_from() == start_at {
                childs.push(edge.get_to());
            }
        }
        if !childs.is_empty() {
            let mut is_extensible = false;
            let prev_paths = paths.clone();
            paths.clear();
            for path in prev_paths {
                let prev_path_len = paths.len();
                if path.last().unwrap() == &start_at {
                    for child in childs.iter() {
                        // path vector is stored or not 
                        if let Some(pos) = path.iter().position(|x| x == child) {
                            if path[pos - 1] != start_at {
                                let mut new_path = path.clone();
                                new_path.push(*child);
                                paths.push(new_path);
                                is_extensible = true;
                            }
                        } else {
                            let mut new_path = path.clone();
                            new_path.push(*child);
                            paths.push(new_path);
                            is_extensible = true;
                        }
                    }
                }
                if paths.len() == prev_path_len {
                    paths.push(path);
                }
            }
            if is_extensible {
                for child in childs {
                    self.find_cfg_paths(child, cfg, paths);
                }
            }
        }
    }

    fn find_state_assignment(&self, link_to: u32) -> bool {
        let state_ids = self.get_states_ids();
        let dfgs = self.network.get_dfgs();
        if state_ids.contains(&link_to) {
            // Get state variable declaration
            // All CFG store the same states => dont need to loop all cfg
            let mut state_var = None; 
            for (_, dfg) in dfgs {
                let actions = dfg.get_new_actions().get(&link_to);
                if let Some(actions) = actions {
                    for action in actions {
                        if let Action::Kill(var, _) = action {
                            if state_var != None {
                                panic!("Unsupported case: where more than 1 assignment, I dont know which one is correct flow");
                            }
                            state_var = Some(var.clone());
                        }
                    }
                }
                break;
            }
            if let Some(state_var) = state_var {
                // From top to bottom 
                // + Find the first KILL(state_variable)
                // + Check whether that kill depends on parameters / msg.sender / states 
                for (_, dfg) in dfgs {
                    let cfg = dfg.get_cfg();
                    let start = cfg.get_start();
                    let mut paths: Vec<Vec<u32>> = vec![];
                    self.find_cfg_paths(start, cfg, &mut paths);
                    for path in paths {
                        for index in (0..path.len()).rev() {
                            let id = path[index];
                            let actions = dfg.get_new_actions().get(&id);
                            if let Some(actions) = actions {
                                for action in actions {
                                    if let Action::Kill(var, _id) = action {
                                        match state_var.contains(var) {
                                            VariableComparison::Equal => {
                                                // TODO
                                            },
                                            VariableComparison::Partial => {
                                                // TODO
                                            },
                                            VariableComparison::NotEqual => {
                                                // TODO
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn find_from(&self, vertex_id: u32) -> Vec<Vec<&DataLink>> {
        let dict = self.network.get_dict();
        let parameter_ids = self.get_parameter_ids();
        let paths = self.network.traverse(vertex_id);
        let address_paths: Vec<Vec<&DataLink>> = paths
            .into_iter()
            .filter(|path| {
                for link in path {
                    match link.get_label() {
                        DataLinkLabel::Internal => {
                            let ref_member = link
                                .get_var()
                                .get_members()
                                .iter()
                                .find(|m| match m {
                                    Member::Reference(_) => true,
                                    _ => false,
                                });
                            if let Some(Member::Reference(ref_id)) = ref_member {
                                let walker = dict.lookup(*ref_id).unwrap();
                                let variable_type = walker.node.attributes["type"].as_str();
                                if let Some(variable_type) = variable_type {
                                    if !(variable_type.starts_with("struct") || variable_type.ends_with("[]") || variable_type == "address"){
                                        return false;
                                    }
                                } else {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        },
                        DataLinkLabel::InFrom(_) => {},
                        DataLinkLabel::OutTo(_) => {},
                        DataLinkLabel::BuiltIn => {},
                        DataLinkLabel::Executor => {},
                    }
                }
                true
            }).collect();
        // Check if
        // + last link points to parameters 
        // + last link points to msg.sender 
        // + address is set to state => state is used to send()/transfer()
        let satisfied_paths: Vec<Vec<&DataLink>> = address_paths
            .into_iter()
            .filter(|path| {
                let link_to = path.last().unwrap().get_to();
                match parameter_ids.contains(&link_to) {
                    true => true,
                    false => self.find_msg_sender(link_to) || self.find_state_assignment(link_to),
                }
            })
        .collect();
        satisfied_paths
    }

    pub fn run(&self) -> Vec<Vec<&DataLink>> {
        let mut satisfied_paths = vec![];
        let dfgs = self.network.get_dfgs();
        let send = Member::Global(String::from("send"));
        let transfer = Member::Global(String::from("transfer"));
        for (_, dfg) in dfgs {
            // Find send / transfer
            let vertices = dfg.get_cfg().get_vertices();
            for vertice in vertices {
                // Functioncall node
                let vertex_id = vertice.get_id();
                if let Some(actions) = dfg.get_new_actions().get(&vertex_id) {
                    for action in actions {
                        if let Action::Use(var, _) = action {
                            let members = var.get_members();
                            // Place where send()/transfer() occurrs 
                            if members.contains(&send) || members.contains(&transfer) {
                                satisfied_paths.append(&mut self.find_from(vertex_id));
                            }
                        }
                    }
                }
            }
        }
        satisfied_paths
    }
}
