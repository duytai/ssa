use std::collections::HashMap;
use crate::core::Walker;

#[derive(Debug)]
pub struct ContractProp {
    id: u32,
    states: Vec<u32>,
    functions: Vec<u32>,
    parents: Vec<u32>,
}

#[derive(Debug)]
pub enum SmartContractQuery {
    FunctionsByContractId(u32),
    StatesByContractId(u32),
}

#[derive(Debug)]
pub struct SmartContract {
    /// contract_id => vec<function_id>
    contracts: HashMap<u32, Vec<u32>>,
    /// contract_id => vec<state_id> 
    states: HashMap<u32, Vec<u32>>,
}

impl SmartContract {
    pub fn new() -> Self {
        SmartContract {
            contracts: HashMap::new(),
            states: HashMap::new(),
        }
    }

    pub fn find(&self, query: SmartContractQuery) -> Option<&Vec<u32>> {
        match query {
            SmartContractQuery::FunctionsByContractId(contract_id) => {
                self.contracts.get(&contract_id)
            },
            SmartContractQuery::StatesByContractId(contract_id) => {
                self.states.get(&contract_id)
            },
        }
    }

    pub fn update(&mut self, contract_walkers: Vec<&Walker>) {
        let mut contracts = HashMap::new();
        for contract_walker in contract_walkers {
            let mut prop = ContractProp {
                id: contract_walker.node.id,
                states: Vec::new(),
                functions: Vec::new(),
                parents: Vec::new(),
            };
            for walker in contract_walker.direct_childs(|_| true).into_iter() {
                match walker.node.name {
                    "InheritanceSpecifier"
                        | "UsingForDirective" => {
                            walker.direct_childs(|_| true)
                                .get(0)
                                .and_then(|walker| walker.node.attributes["referencedDeclaration"].as_u32())
                                .map(|reference| prop.parents.push(reference));
                    },
                    "FunctionDefinition"
                        | "ModifierDefinition" => {
                        prop.functions.push(walker.node.id);
                    },
                    "VariableDeclaration" => {
                        prop.states.push(walker.node.id);
                    },
                    _ => {},
                }
            }
            contracts.insert(contract_walker.node.id, prop);
        }
        // Save entry of functions and states
        for (contract_id, _) in contracts.iter() {
            let mut all_parents = vec![];
            let mut all_functions = vec![];
            let mut all_states = vec![];
            let mut stacks = vec![*contract_id];
            while !stacks.is_empty() {
                stacks.pop().and_then(|contract_id| contracts.get(&contract_id)).map(|prop| {
                    all_parents.push(prop.id);
                    stacks.append(&mut prop.parents.clone());
                });
            }
            for contract_id in all_parents {
                contracts.get(&contract_id).map(|prop| {
                    let mut functions = prop.functions.clone();
                    let mut states = prop.states.clone();
                    all_functions.append(&mut functions);
                    states.reverse();
                    all_states.append(&mut states);
                });
            }
            all_states.reverse();
            self.contracts.insert(*contract_id, all_functions);
            self.states.insert(*contract_id, all_states);
        }
    }
}
