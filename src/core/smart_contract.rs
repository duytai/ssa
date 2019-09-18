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
    StructByName(String),
    ContractByName(String),
}

#[derive(Debug)]
pub struct SmartContract {
    /// contract_id => vec<function_id>
    contracts: HashMap<u32, Vec<u32>>,
    /// contract_id => vec<state_id> 
    states: HashMap<u32, Vec<u32>>,
    /// name => struct_id
    struct_defs: HashMap<String, u32>,
    /// name => contract_id
    contract_defs: HashMap<String, u32>,
}

impl SmartContract {
    pub fn new() -> Self {
        SmartContract {
            contracts: HashMap::new(),
            states: HashMap::new(),
            struct_defs: HashMap::new(),
            contract_defs: HashMap::new(),
        }
    }

    pub fn find(&self, query: SmartContractQuery) -> Option<Vec<u32>> {
        match query {
            SmartContractQuery::FunctionsByContractId(contract_id) => {
                self.contracts.get(&contract_id).map(|x| x.clone())
            },
            SmartContractQuery::StatesByContractId(contract_id) => {
                self.states.get(&contract_id).map(|x| x.clone())
            },
            SmartContractQuery::StructByName(struct_name) => {
                self.struct_defs.get(&struct_name).map(|x| vec![x.clone()])
            },
            SmartContractQuery::ContractByName(contract_name) => {
                self.contract_defs.get(&contract_name).map(|x| vec![x.clone()])
            },
        }
    }

    pub fn update(&mut self, contract_walkers: Vec<&Walker>) {
        let mut contracts = HashMap::new();
        for contract_walker in contract_walkers {
            let mut prop = ContractProp {
                id: contract_walker.node.id,
                states: vec![],
                functions: vec![],
                parents: vec![],
            };
            let contract_name = contract_walker.node.attributes["name"]
                .as_str()
                .unwrap_or("");
            self.contract_defs.insert(contract_name.to_string(), contract_walker.node.id);
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
                    "StructDefinition" => {
                        walker.node.attributes["canonicalName"].as_str().map(|struct_name| {
                            self.struct_defs.insert(struct_name.to_string(), walker.node.id);
                        });
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
        // Save index access entry
    }
}
