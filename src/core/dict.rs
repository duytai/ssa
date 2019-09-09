use crate::core::walker::Walker;
use std::collections::HashMap;

/// Keep inheritance tree and function entry
#[derive(Debug)]
pub struct ContractProp {
    states: Vec<u32>,
    functions: Vec<u32>,
    parents: Vec<u32>,
}

#[derive(Debug)]
pub enum LookupInputType<'a> {
    FunctionId(u32),
    FunctionCallId(u32),
    ContractId(u32),
    ContractName(&'a str),
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
            for walker in walker.direct_childs(|_| true).into_iter() {
                match walker.node.name {
                    "InheritanceSpecifier" | "UsingForDirective" => {
                        let walkers = walker.direct_childs(|_| true);
                        let reference = walkers[0].node.attributes["referencedDeclaration"].as_u32();
                        if let Some(reference) = reference {
                            prop.parents.push(reference);
                        }
                    },
                    "FunctionDefinition" | "ModifierDefinition" => {
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

}
