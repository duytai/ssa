use crate::core::walker::Walker;
use std::collections::HashMap;
use std::collections::HashSet;

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

    /// Find contract constructor
    pub fn lookup_constructor(&self, lookup_input: LookupInputType) -> Option<Walker> {
        if let LookupInputType::ContractId(id) = lookup_input {
            if let Some(walker) = self.lookup(id) {
                return walker.direct_childs(|_| true).into_iter()
                    .find(|w| match w.node.attributes["isConstructor"].as_bool() {
                        Some(true) => true,
                        _ => false,
                    });
            }
        }
        None
    }

    /// Find return statements
    pub fn lookup_returns(&self, id: u32) -> Vec<&Walker> {
        let fi = |walker: &Walker, _: &Vec<Walker>| {
            walker.node.name == "Return" || walker.node.name == "PlaceholderStatement"
        };
        let ig = |_: &Walker, _: &Vec<Walker>| false;
        self.entries
            .get(&id)
            .and_then(|walker| match walker.node.name {
                "FunctionDefinition" | "ModifierDefinition" => {
                    let walkers = walker.walk(true, ig, fi)
                        .iter()
                        .filter_map(|w| self.lookup(w.node.id))
                        .collect::<Vec<&Walker>>();
                    Some(walkers)
                },
                _ => None,
            })
            .unwrap_or(vec![])
    } 

    /// Find all function calls start from @id
    pub fn lookup_function_calls(&self, id: u32) -> Vec<&Walker> {
        let fi = |walker: &Walker, _: &Vec<Walker>| {
            walker.node.name == "FunctionCall" || walker.node.name == "ModifierInvocation"
        };
        let ig = |_: &Walker, _: &Vec<Walker>| false;
        self.entries
            .get(&id)
            .and_then(|walker| {
                let walkers = walker.walk(false, ig, fi)
                    .iter()
                    .filter_map(|w| self.lookup(w.node.id))
                    .collect::<Vec<&Walker>>();
                Some(walkers)
            })
            .unwrap_or(vec![])
    }

    /// Find all parameters of a function definition or function call
    pub fn lookup_parameters(&self, lookup_input: LookupInputType) -> Vec<&Walker> {
        match lookup_input {
            LookupInputType::FunctionId(id) => {
                self.entries.get(&id).and_then(|walker| {
                    let mut ret = vec![];
                    for (index, walker) in walker.direct_childs(|_| true).iter().enumerate() {
                        if index == 0 && walker.node.name == "ParameterList" {
                            for walker in walker.direct_childs(|_| true).iter() {
                                ret.push(walker.node.id);
                            }
                        }
                    }
                    Some(ret)
                }).and_then(|ids| {
                    let ret = ids.iter()
                        .map(|id| { self.lookup(*id)})
                        .filter_map(|w| w)
                        .collect::<Vec<&Walker>>();
                    Some(ret)
                }).unwrap_or(vec![])
            },
            LookupInputType::FunctionCallId(id) => {
                self.entries.get(&id).and_then(|walker| {
                    let mut ret = vec![];
                    for (index, walker) in walker.direct_childs(|_| true).into_iter().enumerate() {
                        if index > 0 {
                            ret.push(walker.node.id)
                        }
                    }
                    Some(ret)
                }).and_then(|ids| {
                    let ret = ids.iter()
                        .map(|id| { self.lookup(*id)})
                        .filter_map(|w| w)
                        .collect::<Vec<&Walker>>();
                    Some(ret)
                }).unwrap_or(vec![])
            },
            _ => vec![],
        }
    }

    /// Find contract based on function call return type
    pub fn lookup_contract(&self, lookup_input: LookupInputType) -> u32 {
        let mut contract_id = None;
        match lookup_input {
            LookupInputType::ContractName(name) => {
                for (id, _) in self.contracts.iter() {
                    self.lookup(*id)
                        .and_then(|walker| walker.node.attributes["name"].as_str())
                        .map(|contract_name| {
                            if name == contract_name {
                                contract_id = Some(*id);
                            }
                        });
                }
            },
            _ => {},
        }
        contract_id.expect("Contract must exists")
    }

    /// Find scoped functions from id of contract
    /// Start from ContractDefinition node
    pub fn lookup_functions(&self, lookup_input: LookupInputType) -> Vec<&Walker> {
        let mut ret = vec![];
        if let LookupInputType::ContractId(id) = lookup_input {
            let mut first_stage_ret = vec![];
            if let Some(prop) = self.contracts.get(&id) {
                for index in (0..prop.functions.len()).rev() {
                    first_stage_ret.push(prop.functions[index]);
                }
                let mut parents = prop.parents.clone();
                loop {
                    match parents.pop() {
                        Some(contract_id) => {
                            if let Some(prop) = self.contracts.get(&contract_id) {
                                for index in (0..prop.functions.len()).rev() {
                                    first_stage_ret.push(prop.functions[index]);
                                }
                                parents.extend_from_slice(&prop.parents[..]);
                            }
                        },
                        None => { break; }
                    }
                }
            }
            first_stage_ret.reverse();
            ret = first_stage_ret.iter()
                .map(|id| { self.lookup(*id)})
                .filter_map(|w| w)
                .collect::<Vec<&Walker>>();
            // Find whether a contract is initialized
            if let Some(walker) = self.lookup(id) {
                let mut contract_ids = HashSet::new();
                let fi = |walker: &Walker, _: &Vec<Walker>| {
                    let type_attr = walker.node.attributes["type"].as_str();
                    match type_attr {
                        Some(type_attr) => walker.node.name == "UserDefinedTypeName" && type_attr.starts_with("contract"),
                        None => false,
                    }
                };
                let ig = |_: &Walker, _: &Vec<Walker>| false;
                for walker in walker.walk(false, ig, fi) {
                    let contract_id = walker.node.attributes["referencedDeclaration"].as_u32().unwrap();
                    if contract_id != id {
                        contract_ids.insert(contract_id);
                    }
                }
                for contract_id in contract_ids {
                    ret.append(&mut self.lookup_functions(LookupInputType::ContractId(contract_id)));
                }
            }
        }
        ret
    }

    /// Find a list of states from function_id
    /// Include inherited states
    pub fn lookup_states(&self, lookup_input: LookupInputType) -> Vec<&Walker> {
        let mut ret = vec![];
        match lookup_input {
            LookupInputType::FunctionId(id) => {
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
            },
            LookupInputType::ContractId(id) => {
                if let Some(prop) = self.contracts.get(&id) {
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
                }
            },
            _ => {},
        }
        ret.reverse();
        ret.iter()
           .map(|id| { self.lookup(*id)})
           .filter_map(|w| w)
           .collect::<Vec<&Walker>>()
    }
}
