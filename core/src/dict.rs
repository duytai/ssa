use crate::walker::Walker;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ContractProp {
    states: Vec<u32>,
    functions: Vec<u32>,
    parents: Vec<u32>,
}

#[derive(Debug)]
pub struct Dictionary<'a> {
    entries: HashMap<u32, Walker<'a>>,
    contracts: HashMap<u32, ContractProp>,
}

impl<'a> Dictionary<'a> {
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
            walker.for_each(|walker, _| {
                match walker.node.name {
                    "FunctionDefinition" => {
                        prop.functions.push(walker.node.id);
                    },
                    "VariableDeclaration" => {
                        prop.states.push(walker.node.id);
                    },
                    _ => {},
                }
            });
            self.contracts.insert(walker.node.id, prop);
        }
        walker.for_each(|walker, _| {
            self.traverse(&walker);
            self.entries.insert(walker.node.id, walker);
        });
    }

    pub fn lookup(&self, id: u32) -> Option<&Walker> {
        self.entries.get(&id)
    }

    pub fn lookup_states(&self, id: u32) -> Vec<&Walker> {
        let mut ret = vec![];
        for (_, prop) in self.contracts.iter() {
            if prop.functions.contains(&id) {
                ret.extend_from_slice(&prop.states[..]);
                let mut parents = prop.parents.clone();
                loop {
                    match parents.pop() {
                        Some(contract_id) => {
                            if let Some(prop) = self.contracts.get(&contract_id) {
                                ret.extend_from_slice(&prop.states[..]);
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
