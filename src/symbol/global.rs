use crate::walker::Walker;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Signature {
    None,
    Variable(String),
    Function((Vec<String>, Vec<String>)),
}

#[derive(Debug)]
pub enum Definition {
    Plain(Vec<(String, Signature)>),
    Ref(Vec<String>),
}

#[derive(Debug)]
pub struct GlobalTable {
    definitions: HashMap<String, Definition>,
}

impl GlobalTable {
    pub fn new(value: &json::JsonValue, source: &str) -> Self {
        let walker = Walker::new(value, source);
        let mut r = GlobalTable { definitions: HashMap::new() };
        r.traverse(&walker);
        r
    }

    pub fn traverse(&mut self, walker: &Walker) {
        walker.all_break(|walker| {
            walker.node.name == "ContractDefinition"
        }, |walkers| {
            for walker in walkers.iter() {
                let contract_name = walker.node.attributes["name"].as_str().unwrap().to_string();
                let contract_kind = walker.node.attributes["contractKind"].as_str().unwrap().to_string();
                let mut contract_props = vec![];
                let mut usings: HashMap<String, Vec<String>> = HashMap::new();
                let mut parents: Vec<String> = vec![];
                walker.for_each(|walker, _| {
                    match walker.node.name {
                        "EnumDefinition" => {
                            let enum_name = walker.node.attributes["name"].as_str().unwrap().to_string();
                            let mut props = vec![];
                            walker.for_each(|walker, _| {
                                let var_name = walker.node.attributes["name"].as_str().unwrap().to_string();
                                props.push((var_name, Signature::None));
                            });
                            let key = format!("enum {}.{}", contract_name, enum_name);
                            self.definitions.insert(key, Definition::Plain(props));
                        },
                        "StructDefinition" => {
                            let struct_name = walker.node.attributes["name"].as_str().unwrap().to_string();
                            let mut props = vec![];
                            walker.for_each(|walker, _| {
                                let var_name = walker.node.attributes["name"].as_str().unwrap().to_string();
                                let var_type = walker.node.attributes["type"].as_str().unwrap().to_string(); 
                                props.push((var_name, Signature::Variable(var_type)));
                            });
                            let key = format!("struct {}.{}", contract_name, struct_name);
                            self.definitions.insert(key, Definition::Plain(props));
                        },
                        "EventDefinition" => {
                            let event_name = walker.node.attributes["name"].as_str().unwrap().to_string();
                            let mut props = vec![];
                            walker.all_break(|walker| {
                                walker.node.name == "VariableDeclaration"
                            }, |walkers| {
                                for walker in walkers {
                                    let var_name = walker.node.attributes["name"].as_str().unwrap().to_string();
                                    let var_type = walker.node.attributes["type"].as_str().unwrap().to_string(); 
                                    props.push((var_name, Signature::Variable(var_type)));
                                }
                            });
                            let key = format!("event {}.{}", contract_name, event_name);
                            self.definitions.insert(key, Definition::Plain(props));
                        },
                        "VariableDeclaration" => {
                            let var_name = walker.node.attributes["name"].as_str().unwrap().to_string();
                            let var_type = walker.node.attributes["type"].as_str().unwrap().to_string();
                            contract_props.push((var_name, Signature::Variable(var_type)));
                        },
                        "FunctionDefinition" => {
                            let func_name = walker.node.attributes["name"].as_str().unwrap().to_string();
                            let mut returns = vec![];
                            let mut params = vec![];
                            walker.for_each(|walker, index| {
                                if walker.node.name == "ParameterList" {
                                    match index {
                                        0 => {
                                            walker.for_each(|walker, _| {
                                                let var_type = walker.node.attributes["type"].as_str().unwrap().to_string();
                                                params.push(var_type);
                                            });
                                        },
                                        1 => {
                                            walker.for_each(|walker, _| {
                                                let var_type = walker.node.attributes["type"].as_str().unwrap().to_string();
                                                returns.push(var_type);
                                            });
                                        },
                                        _ => {},
                                    }
                                }
                            });
                            contract_props.push((func_name, Signature::Function((params, returns))));
                        },
                        "UsingForDirective" => {
                            let mut math = String::from("");
                            let mut var_type = String::from("");
                            walker.for_each(|walker, index| {
                                let temp = walker.node.attributes["type"].as_str().unwrap().to_string();
                                if index == 0 { 
                                    math = temp;
                                } else {
                                    var_type = temp;
                                }
                            });
                            if let Some(maths) = usings.get_mut(&var_type) {
                                (*maths).push(math);
                            } else {
                                usings.insert(var_type, vec![math]);
                            }
                        },
                        "InheritanceSpecifier" => {
                            walker.for_each(|walker, index| {
                                let var_type = walker.node.attributes["type"].as_str().unwrap().to_string();
                                parents.push(var_type);
                            });
                        },
                        _ => unimplemented!(),
                    }
                });
                let key = format!("{} {}", contract_kind, contract_name);
                self.definitions.insert(key, Definition::Plain(contract_props));
                for (var_type, maths) in usings {
                    let key = format!("{} {}", var_type, contract_name);
                    self.definitions.insert(key, Definition::Ref(maths));
                }
                if !parents.is_empty() {
                    let key = format!("parent {}", contract_name);
                    self.definitions.insert(key, Definition::Ref(parents));
                }
            }
        });
    }
}
