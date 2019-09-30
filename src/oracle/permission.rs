use crate::dfg::Network;
use std::collections::HashSet;
use std::collections::HashMap;
use crate::core::{
    SmartContractQuery,
    Action,
    Member,
    Variable,
    VariableComparison,
};

pub struct Permission {
    owner_variables: HashSet<Variable>,
}

impl Permission {
    pub fn new(network: &Network) -> Self {
        let mut permission = Permission { owner_variables: HashSet::new() };
        permission.update(network);
        permission
    }

    pub fn update(&mut self, network: &Network) {
        let mut all_actions = HashMap::new();
        let mut state_sources = vec![];
        let dict = network.get_dict();
        let contract_id = network.get_contract_id();
        let mut constructor_vertices = vec![];
        for (function_id, dfg) in network.get_dfgs().iter() {
            all_actions.extend(dfg.get_new_actions());
            let walker = dict.walker_at(*function_id).unwrap();
            let is_constructor = walker.node.attributes["isConstructor"].as_bool().unwrap_or(false);
            if is_constructor {
                for vertex in dfg.get_cfg().get_vertices() {
                    constructor_vertices.push(vertex.get_id());
                }
            }
        }
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
        for state_id in dict.find_ids(SmartContractQuery::StatesByContractId(contract_id)) {
            for variable in get_variables(state_id) {
                for vertex_id in constructor_vertices.iter() {
                    state_sources.push((variable.clone(), *vertex_id));
                }
            }
        }
        let mut msg_sender_variables = vec![];
        let msg_sender = vec![
            vec!["msg", "sender", "callcode", "bool"],
            vec!["msg", "sender", "transfer", "void"],
            vec!["msg", "sender", "balance", "uint256"],
            vec!["msg", "sender", "delegatecall", "bool"],
            vec!["msg", "sender", "send", "bool"],
            vec!["msg", "sender", "call", "bool"],
        ];
        for sender in msg_sender {
            let members = vec![
                Member::Global(sender[0].to_string()),
                Member::Global(sender[1].to_string()),
                Member::Global(sender[2].to_string()),
            ];
            let variable = Variable::new(
                members,
                sender[..3].join("."),
                sender[3].to_string(),
            );
            msg_sender_variables.push(variable);
        }
        for state_source in state_sources {
            let excution_paths = network.traverse(state_source);
            for excution_path in excution_paths {
                if excution_path.len() > 1 {
                    let (variable, _) = excution_path.last().unwrap();
                    for msg_sender_variable in msg_sender_variables.iter() {
                        let comp = msg_sender_variable.contains(variable);
                        if comp == VariableComparison::Equal || comp == VariableComparison::Partial {
                            let (owner_variable, _) = excution_path.first().unwrap();
                            self.owner_variables.insert(owner_variable.clone());
                        }
                    }
                }
            } 
        }
    }

    pub fn get_owner_variables(&self) -> &HashSet<Variable> {
        &self.owner_variables
    }
}
