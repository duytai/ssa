use crate::walker::Walker;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Signature {
    None,
    Variable(String),
    Function(String),
}

#[derive(Debug)]
pub struct GlobalTable {
    table: HashMap<String, Vec<(String, Signature)>>,
}

impl GlobalTable {
    pub fn new(value: &json::JsonValue, source: &str) -> Self {
        let walker = Walker::new(value, source);
        let mut r = GlobalTable { table: HashMap::new() };
        r.traverse(&walker);
        r
    }

    pub fn traverse(&mut self, walker: &Walker) {
        walker.all_break(|walker| {
            walker.node.name == "ContractDefinition"
        }, |walkers| {
            for walker in walkers {
                let contract_name = walker.node.attributes["name"].as_str().unwrap().to_string();
                let contract_kind = walker.node.attributes["contractKind"].as_str().unwrap().to_string();
                let mut contract_props = vec![];
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
                            self.table.insert(key, props);
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
                            self.table.insert(key, props);
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
                            self.table.insert(key, props);
                        },
                        "VariableDeclaration" => {
                            let var_name = walker.node.attributes["name"].as_str().unwrap().to_string();
                            let var_type = walker.node.attributes["type"].as_str().unwrap().to_string();
                            contract_props.push((var_name, Signature::Variable(var_type)));
                        },
                        "FunctionDefinition" => {
                            let func_name = walker.node.attributes["name"].as_str().unwrap().to_string();
                            let visibility = walker.node.attributes["visibility"].as_str().unwrap().to_string();
                            let mut returns = vec![];
                            walker.for_each(|walker, index| {
                                if walker.node.name == "ParameterList" && index == 1 {
                                    walker.for_each(|walker, _| {
                                        let var_type = walker.node.attributes["type"].as_str().unwrap().to_string();
                                        returns.push(var_type);
                                    });
                                }
                            });
                            let signature = format!("({})", returns.join(","));
                            contract_props.push((func_name, Signature::Function(signature)));
                        },
                        "UsingForDirective" => {},
                        "InheritanceSpecifier" => {},
                        _ => unimplemented!(),
                    }
                });
                let key = format!("{} {}", contract_kind, contract_name);
                self.table.insert(key, contract_props);
            }
        });
    }
}
