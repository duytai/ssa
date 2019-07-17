use std::collections::HashSet;
use crate::dfg::Network;
use crate::cfg::ControlFlowGraph;
use crate::core::{
    Action,
    DataLinkLabel,
    DataLink,
    Member,
    VariableComparison,
    Variable,
};

/// Detect gasless send <X>.send() / <X>.transfer()
/// - address <X> depends on parameters
///  + has no condition check on path to <X> 
/// - address <X> depends on msg.sender
///  + has no condition check on path to <X> 
pub struct GaslessSend<'a> {
    network: &'a Network<'a>,
}

pub enum GaslessSendResult<'a> {
    DirectUse(Variable),
    LinkedUse(Vec<&'a DataLink>),
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

    fn is_address_var(&self, var: &Variable) -> bool {
        let dict = self.network.get_dict();
        var.get_members()
        .iter()
        .find(|m| match m {
            Member::Reference(_) => true,
            _ => false,
        })
        .and_then(|ref_member| match ref_member {
            Member::Reference(ref_id) => {
                Some(ref_id)
            },
            _ => None,
        })
        .and_then(|ref_id| dict.lookup(*ref_id))
        .and_then(|walker| walker.node.attributes["type"].as_str())
        .and_then(|variable_type| {
            Some(variable_type.starts_with("struct") || variable_type == "address[]" || variable_type == "address")
        })
        .unwrap_or(false)
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
            let mut kill_state_ids: HashSet<u32> = HashSet::new();
            for (_, dfg) in dfgs {
                let actions = dfg.get_new_actions().get(&link_to);
                if let Some(actions) = actions {
                    for action in actions {
                        if let Action::Kill(var, _) = action {
                            if self.is_address_var(var) {
                                state_var = Some(var.clone());
                            }
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
                    for mut path in paths {
                        path.reverse();
                        let pos = path.iter().position(|id| {
                            // Current id is not state var 
                            if id != &link_to {
                                let actions = dfg.get_new_actions().get(&id);
                                if let Some(actions) = actions {
                                    for action in actions {
                                        if let Action::Kill(var, _id) = action {
                                            match state_var.contains(var) {
                                                VariableComparison::Equal | VariableComparison::Partial => {
                                                    // Var is killed here
                                                    if self.is_address_var(var) {
                                                        return true;
                                                    } 
                                                },
                                                VariableComparison::NotEqual => {}
                                            }
                                        }
                                    }
                                }
                            }
                            false
                        });
                        if let Some(pos) = pos {
                            kill_state_ids.insert(path[pos]);
                        }
                    }
                }
            }
            // Search back to see whether it depends on parameters or msg.sender
            for id in kill_state_ids {
                // TODO
                let satisfied_paths = self.find_from(id);
                println!("satisfied_paths: {:?}", satisfied_paths);
            } 
        }
        false
    }

    fn find_from(&self, vertex_id: u32) -> Vec<Vec<&DataLink>> {
        let parameter_ids = self.get_parameter_ids();
        let paths = self.network.traverse(vertex_id);
        let address_paths: Vec<Vec<&DataLink>> = paths
            .into_iter()
            .filter(|path| {
                for link in path {
                    match link.get_label() {
                        DataLinkLabel::Internal => {
                            if !self.is_address_var(link.get_var()) {
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

    pub fn run(&self) -> Vec<GaslessSendResult> {
        let mut ret = vec![];
        let mut linked_uses = vec![];
        let mut direct_uses = vec![];
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
                                // msg.sender.send / msg.sender.transfer
                                if vec!["msg.sender.send", "msg.sender.transfer"].contains(&var.get_source()) {
                                    direct_uses.push(var.clone());
                                }
                                linked_uses.append(&mut self.find_from(vertex_id));
                            }
                        }
                    }
                }
            }
        }
        for p in linked_uses {
            ret.push(GaslessSendResult::LinkedUse(p));
        }
        for p in direct_uses {
            ret.push(GaslessSendResult::DirectUse(p));
        }
        ret
    }
}
