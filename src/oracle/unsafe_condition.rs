use crate::dfg::Network;
use crate::core::Action;
use crate::core::Member;
use crate::core::Shape;
use std::collections::HashMap;
use std::collections::HashSet;

/// How to check:
/// execution_path contains send/transfer/delegatecall/call/callcode/selfdestruct/suicide
/// block.number/block.timestamp is used in these functions
/// condition in execution_path use block.number/block.timestamp directly or depend on them 
///
/// block.numer is saved to variable
/// invoke function call
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
        let sending_members = vec![
            Member::Global(String::from("send")),
            Member::Global(String::from("transfer")),
            Member::Global(String::from("call")),
            Member::Global(String::from("callcode")),
            Member::Global(String::from("delegatecall")),
            Member::Global(String::from("selfdestruct")),
            Member::Global(String::from("suicide")),
        ];
        let all_actions = network.get_all_actions();
        let all_vertices = network.get_all_vertices();
        let mut all_state_variables = HashMap::new();
        for variable in network.get_all_states() {
            all_state_variables.insert(variable, variable);
        }
        for (_, dfg) in network.get_dfgs() {
            let cfg = dfg.get_cfg();
            let execution_paths = cfg.get_execution_paths();
            let mut cfg_depend_tups: HashSet<(u32, u32)> = HashSet::new();
            for execution_path in execution_paths {
                let mut path_state_variables = all_state_variables.clone();
                let mut state_related_vertices: HashSet<u32> = HashSet::new();
                for vertex_id in execution_path.iter().rev() {
                    // Control flow dependency
                    if !state_related_vertices.is_empty() {
                        let vertice = all_vertices.get(vertex_id).unwrap();
                        if vertice.is_condition() {
                            for id in state_related_vertices.iter() {
                                cfg_depend_tups.insert((*vertex_id, id.clone()));
                            }
                        }
                    }
                    // State variables are killed 
                    if let Some(actions) = all_actions.get(vertex_id) {
                        for action in actions {
                            if let Action::Kill(variable, _) = action {
                                if path_state_variables.contains_key(variable) {
                                    path_state_variables.remove(variable);
                                    state_related_vertices.insert(*vertex_id);
                                }
                            }
                        }
                    }
                }
            }
            for (from, to) in cfg_depend_tups {
                println!("{} - {}", from, to);
                if cfg.is_control_dependency(from, to, vec![]) {
                    println!("OK: {} - {}", from, to);
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
                // for variable in network.get_variables(vertex_id) {
                    // let members = variable.get_members();
                    // let vertice = all_vertices.get(vertex_id).unwrap();
                    // let is_send = members.iter().fold(false, |acc, m| {
                        // acc || (sending_members.contains(m) && vertice.is_function_call())
                    // });
                    // if is_send {
                        // FIND conditions
                    // }
                // }
