use crate::dfg::Network;
use crate::core::Action;
use crate::core::Member;
use crate::core::Variable;
use crate::core::Vertex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

/// How to check:
/// execution_path contains send/transfer/delegatecall/call/callcode/selfdestruct/suicide
/// block.number/block.timestamp is used in these functions
/// condition in execution_path use block.number/block.timestamp directly or depend on them 
///
/// block.numer is saved to variable
/// invoke function call
pub struct UnsafeSendingCondition {
}

impl UnsafeSendingCondition {
    pub fn new(network: &Network) -> Self {
        let mut unsafe_sending_condition = UnsafeSendingCondition {
        };
        unsafe_sending_condition.update(network);
        unsafe_sending_condition
    }
    
    fn is_blocknumber(&self, variable: &Variable) -> bool {
        let source = variable.get_source();
        source.starts_with("block.number")
    }

    fn is_timestamp(&self, variable: &Variable) -> bool {
        let source = variable.get_source();
        source.starts_with("block.timestamp") || source.starts_with("now")
    }

    fn is_send(&self, variable: &Variable, vertice: &Vertex) -> bool {
        let sending_members = vec![
            Member::Global(String::from("send")),
            Member::Global(String::from("transfer")),
            Member::Global(String::from("call")),
            Member::Global(String::from("callcode")),
            Member::Global(String::from("delegatecall")),
            Member::Global(String::from("selfdestruct")),
            Member::Global(String::from("suicide")),
        ];
        let members = variable.get_members();
        let is_send = members.iter().fold(false, |acc, m| {
            acc || (sending_members.contains(m) && vertice.is_function_call())
        });
        is_send
    }

    fn update(&mut self, network: &Network) {
        let all_actions = network.get_all_actions();
        let all_vertices = network.get_all_vertices();
        let mut all_control_dependency: HashMap<Variable, HashSet<u32>> = HashMap::new();
        let mut all_state_variables = HashMap::new();
        for variable in network.get_all_states() {
            all_state_variables.insert(variable, variable);
        }
        for (_, dfg) in network.get_dfgs() {
            let cfg = dfg.get_cfg();
            let execution_paths = cfg.get_execution_paths();
            for execution_path in execution_paths {
                let mut state_variables = all_state_variables.clone();
                let mut control_dependency: HashMap<Variable, Vec<u32>> = HashMap::new();
                for vertex_id in execution_path.iter().rev() {
                    // Control flow dependency
                    if !control_dependency.is_empty() {
                        let vertice = all_vertices.get(vertex_id).unwrap();
                        if vertice.is_condition() {
                            for (state_variable, condition_ids) in control_dependency.clone() {
                                let state_kill_at = condition_ids.first().unwrap();
                                if cfg.is_control_dependency(*vertex_id, *state_kill_at, vec![]) {
                                    if let Some(condition_ids) = control_dependency.get_mut(&state_variable) {
                                        condition_ids.push(*vertex_id);
                                    }
                                }
                            }
                        }
                    }
                    // State variables are killed
                    if let Some(actions) = all_actions.get(vertex_id) {
                        for action in actions {
                            if let Action::Kill(state_variable, _) = action {
                                if state_variables.contains_key(state_variable) {
                                    state_variables.remove(state_variable);
                                    control_dependency.insert(state_variable.clone(), vec![*vertex_id]);
                                }
                            }
                        }
                    }
                }
                // Store control_dependency
                for (state_variable, condition_ids) in control_dependency {
                    if let Some(all_control_ids) = all_control_dependency.get_mut(&state_variable) {
                        all_control_ids.extend(condition_ids);
                    } else {
                        let all_control_ids: HashSet<u32> = HashSet::from_iter(condition_ids.into_iter());
                        all_control_dependency.insert(state_variable, all_control_ids);
                    }
                }
            }
        }
        // Find dependency in condition node 
        let mut all_state_dependency = HashMap::new(); 
        for (state_variable, condition_ids) in all_control_dependency {
            let mut timestamp = false;
            let mut blocknumber = false;
            let mut root_variables = HashSet::new();
            for condition_id in condition_ids {
                for variable in network.get_variables(&condition_id) {
                    timestamp = timestamp || self.is_timestamp(&variable);
                    blocknumber = blocknumber || self.is_blocknumber(&variable);
                    for depend_path in network.traverse((variable, condition_id)) {
                        if depend_path.len() > 1 {
                            let (root_variable, _) = depend_path.last().unwrap();
                            blocknumber = blocknumber || self.is_blocknumber(root_variable);
                            timestamp = timestamp || self.is_timestamp(root_variable);
                            if all_state_variables.contains_key(root_variable) {
                                root_variables.insert(root_variable.clone());
                            }
                        }
                    }
                }
            }
            all_state_dependency.insert(
                state_variable,
                (root_variables, timestamp, blocknumber)
            );
        }
        println!("----all-state-dependency----");
        for (k, v) in all_state_dependency {
            println!("-----");
            println!("k: {:?}", k);
            println!("v: {:?}", v);
        } 
    }
}
                // for variable in network.get_variables(vertex_id) {
                // }
