use std::collections::HashMap;
use std::collections::HashSet;
use crate::core::Action;
use crate::dfg::Network;

pub struct CriticalStatement {
}

impl CriticalStatement {
    pub fn new(network: &Network) -> Self {
        let cs = CriticalStatement {};
        cs.find_statements(network);
        cs
    }

    fn find_statements(&self, network: &Network) {
        let mut all_actions = HashMap::new();
        for (_, dfg) in network.get_dfgs().iter() {
            all_actions.extend(dfg.get_new_actions());
        }
        for (vertex_id, actions) in all_actions {
            let mut variables = HashSet::new();
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
            // println!("vertex_id: {:?}", vertex_id);
            // println!("variables: {:?}", variables);
        }
    }
}
