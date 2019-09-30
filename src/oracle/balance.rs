use crate::dfg::Network;
use crate::core::{
    Action,
    Variable,
    Member,
    SmartContractQuery,
    VariableComparison,
};
use std::collections::{ HashSet, HashMap };

pub struct Balance {
    msg_value_variables: HashSet<Variable>,
}

impl Balance {
    pub fn new(network: &Network) -> Self {
        let mut balance = Balance { msg_value_variables: HashSet::new() };
        balance.update(network);
        balance
    }

    fn update(&mut self, network: &Network) {
        let mut all_actions = HashMap::new();
        let mut all_vertices = vec![]; 
        let mut state_sources = vec![];
        let dict = network.get_dict();
        let contract_id = network.get_contract_id();

        for (_, dfg) in network.get_dfgs().iter() {
            all_actions.extend(dfg.get_new_actions());
            for vertex in dfg.get_cfg().get_vertices() {
                all_vertices.push(vertex.get_id());
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
                for vertex_id in all_vertices.iter() {
                    state_sources.push((variable.clone(), *vertex_id));
                }
            }
        }
        let members = vec![
            Member::Global(String::from("msg")),
            Member::Global(String::from("value")),
        ];
        let msg_value_variable = Variable::new(
            members,
            String::from("msg.value"),
            String::from("uint256"),
        );
        for state_source in state_sources {
            let excution_paths = network.traverse(state_source);
            for excution_path in excution_paths {
                if excution_path.len() > 1 {
                    let (variable, _) = excution_path.last().unwrap();
                    let comp = msg_value_variable.contains(variable);
                    if comp == VariableComparison::Equal || comp == VariableComparison::Partial {
                        let (balance_variable, _) = excution_path.first().unwrap();
                        self.msg_value_variables.insert(balance_variable.clone());
                    }
                }
            }
        }
    }

    pub fn get_msg_value_variables(&self) -> &HashSet<Variable> {
        &self.msg_value_variables
    }
}
