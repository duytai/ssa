use std::collections::HashSet;
use crate::dfg::Network;
use crate::core::{
    Action,
    LookupInputType,
    VariableComparison,
};

pub struct Permission<'a> {
    network: &'a Network<'a>, 
}

impl<'a> Permission<'a> {
    pub fn new(network: &'a Network) -> Self {
        Permission { network }
    }

    /// Which state variables containing msg.sender address
    fn find_owner_states(&self) {
        let dict = self.network.get_dict();
        let entry_id = self.network.get_entry_id();
        let lookup_input = LookupInputType::ContractId(entry_id);
        let state_walkers = dict.lookup_states(lookup_input);
        let mut killed_states = vec![];
        for (id, dfg) in self.network.get_dfgs() {
            if let Some(walker) = dict.lookup(*id) {
                let is_constructor = walker.node
                    .attributes["isConstructor"]
                    .as_bool()
                    .unwrap_or(false);
                if is_constructor {
                    let mut state_vars = vec![];
                    let mut state_ids = HashSet::new();
                    for state_walker in state_walkers.iter() {
                        state_ids.insert(state_walker.node.id);
                        if let Some(actions) = dfg.get_new_actions().get(&state_walker.node.id) {
                            for action in actions {
                                if let Action::Kill(var, _) = action {
                                    if var.get_type() == &Some(String::from("address")) {
                                        state_vars.push(var);
                                    }
                                }
                            }
                        }
                    }
                    for vertice in dfg.get_cfg().get_vertices() {
                        if state_ids.contains(&vertice.get_id()) {
                            continue;
                        }
                        if let Some(actions) = dfg.get_new_actions().get(&vertice.get_id()) {
                            for action in actions {
                                if let Action::Kill(var, kill_at) = action {
                                    for state_var in state_vars.iter() {
                                        if var.contains(state_var) == VariableComparison::Equal {
                                            killed_states.push((var, kill_at));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        for (state_var, kill_at) in killed_states {
            println!("state_var: {:?}", state_var);
            for dependent_path in self.network.traverse(*kill_at) {
                println!(">> dependent_path");
                for link in dependent_path {
                    println!("\t {:?}", link);
                }
                break;
            }
        }
    }
    
    pub fn create_table(&self) {
        self.find_owner_states();
    }
}
