use crate::core::walker::Walker;
use std::collections::HashMap;

/// Keep inheritance tree and function entry
#[derive(Debug)]
pub struct ContractProp {
    states: Vec<u32>,
    functions: Vec<u32>,
    parents: Vec<u32>,
}

/// Allow searching by node id 
#[derive(Debug)]
pub struct Dictionary<'a> {
    entries: HashMap<u32, Walker<'a>>,
    contracts: HashMap<u32, ContractProp>,
}

impl<'a> Dictionary<'a> {
    /// Create new dictionary
    pub fn new(value: &'a json::JsonValue, sources: &'a HashMap<String, String>) -> Self {
        let mut dict = Dictionary {
            entries: HashMap::new(),
            contracts: HashMap::new(),
        };
        for (name, source) in sources {
            let ast_one = &value["sources"][name]["AST"];
            let walker = Walker::new(ast_one, source);
            dict.traverse(&walker);
        }
        dict
    }

    /// Traverse AST and save data for later searches
    pub fn traverse(&mut self, walker: &Walker<'a>) {
        if walker.node.name == "ContractDefinition" {
            let mut prop = ContractProp {
                states: Vec::new(),
                functions: Vec::new(),
                parents: Vec::new(),
            };
            for m in walker.node.attributes["contractDependencies"].members() {
                if !m.is_null() {
                    if let Some(m) = m.as_u32() {
                        prop.parents.push(m);
                    }
                }
            }
            for walker in walker.direct_childs(|_| true).into_iter() {
                match walker.node.name {
                    "FunctionDefinition" => {
                        prop.functions.push(walker.node.id);
                    },
                    "VariableDeclaration" => {
                        prop.states.push(walker.node.id);
                    },
                    _ => {},
                }
            }
            self.contracts.insert(walker.node.id, prop);
        }
        for walker in walker.direct_childs(|_| true).into_iter() {
            self.traverse(&walker);
            self.entries.insert(walker.node.id, walker);
        }
    }

    /// Find walker by node id
    pub fn lookup(&self, id: u32) -> Option<&Walker> {
        self.entries.get(&id)
    }

    /// Find a list of functions by node id, the list includes inherited functions
    pub fn lookup_states(&self, id: u32) -> Vec<&Walker> {
        let mut ret = vec![];
        for (_, prop) in self.contracts.iter() {
            if prop.functions.contains(&id) {
                for index in (0..prop.states.len()).rev() {
                    ret.push(prop.states[index]);
                }
                let mut parents = prop.parents.clone();
                loop {
                    match parents.pop() {
                        Some(contract_id) => {
                            if let Some(prop) = self.contracts.get(&contract_id) {
                                for index in (0..prop.states.len()).rev() {
                                    ret.push(prop.states[index]);
                                }
                                parents.extend_from_slice(&prop.parents[..]);
                            }
                        },
                        None => { break; }
                    }
                }
                break;
            }
        }
        ret.reverse();
        ret.iter()
           .map(|id| { self.lookup(*id)})
           .filter_map(|w| w)
           .collect::<Vec<&Walker>>()
    }
}
