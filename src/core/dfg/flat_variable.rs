extern crate regex;
use regex::Regex;
use crate::core::{
    Dictionary,
    SmartContractQuery,
    Member,
    Walker,
    Utils,
};

pub struct FlatVariable<'a> {
    dict: &'a Dictionary<'a>,
    flats: Vec<(Vec<Member>, String, String)>,
}

impl<'a> FlatVariable<'a> {
    pub fn new(walker: &Walker, dict: &'a Dictionary) -> Self {
        let mut flat_variable = FlatVariable { dict, flats: vec![] };
        flat_variable.update(&Utils::find_root_kind(walker, dict), vec![], vec![]);
        for flat in flat_variable.flats.iter() {
            println!("\t{:?}", flat);
        }
        flat_variable
    }


    fn update(&mut self, kind: &str, mut members: Vec<Member>, mut attributes: Vec<String>) {
        let struct_regex = Regex::new(r"^struct ([^\[\]]*)((\[\])*)").unwrap();
        let mapping_regex = Regex::new(r"^mapping\(([^=>]+)\s+=>\s+([^=>]+)\)((\[\])*)").unwrap();
        let contract_regex = Regex::new(r"^contract ([^\[\]]*)((\[\])*)").unwrap();
        let matches = (struct_regex.is_match(kind), mapping_regex.is_match(kind), contract_regex.is_match(kind));
        match matches {
            (true, _, _) => {
                for cap in struct_regex.captures_iter(kind) {
                    let struct_kind = (&cap[1]).to_string();
                    let dimension = (&cap[2]).len() / 2;
                    for _ in 0..dimension {
                        members.push(Member::IndexAccess);
                        attributes.push(String::from("$"));
                    }
                    self.dict.find_walkers(SmartContractQuery::StructByName(struct_kind)).get(0).map(|walker| {
                        for walker in walker.direct_childs(|_| true) {
                            let mut members = members.clone();
                            let mut attributes = attributes.clone();
                            let name = walker.node.attributes["name"].as_str().unwrap_or("");
                            members.push(Member::Reference(walker.node.id));
                            attributes.push(name.to_string());
                            self.update(&Utils::normalize_kind(&walker), members, attributes);
                        }
                    });
                }
            },
            (_, true, _) => {
                for cap in mapping_regex.captures_iter(kind) {
                    let mapping_kind = (&cap[2]).to_string();
                    let dimension = (&cap[3]).len() / 2 + 1;
                    for _ in 0..dimension {
                        members.push(Member::IndexAccess);
                        attributes.push(String::from("$"));
                    }
                    self.update(&mapping_kind, members.clone(), attributes.clone());
                }
            },
            (_, _, true) => {
                for cap in contract_regex.captures_iter(kind) {
                    let contract_kind = (&cap[1]).to_string();
                    let dimension = (&cap[2]).len() / 2;
                    for _ in 0..dimension {
                        members.push(Member::IndexAccess);
                        attributes.push(String::from("$"));
                    }
                    self.dict.find_walkers(SmartContractQuery::ContractByName(contract_kind)).get(0).map(|walker| {
                        for walker in walker.direct_childs(|_| true) {
                            match walker.node.name {
                                "VariableDeclaration" => {
                                    let mut members = members.clone();
                                    let mut attributes = attributes.clone();
                                    let name = walker.node.attributes["name"].as_str().unwrap_or("");
                                    members.push(Member::Reference(walker.node.id));
                                    attributes.push(name.to_string());
                                    self.update(&Utils::normalize_kind(&walker), members, attributes);
                                },
                                "FunctionDefinition" => {
                                    let mut members = members.clone();
                                    let mut attributes = attributes.clone();
                                    let name = walker.node.attributes["name"].as_str().unwrap_or("");
                                    members.push(Member::Reference(walker.node.id));
                                    attributes.push(name.to_string());
                                    walker.direct_childs(|_| true).get(1).map(|walker| {
                                        walker.direct_childs(|_| true).get(0).map(|walker| {
                                            let name = walker.node.attributes["name"].as_str().unwrap_or("");
                                            members.push(Member::Reference(walker.node.id));
                                            attributes.push(name.to_string());
                                            self.update(&Utils::normalize_kind(&walker), members, attributes);
                                        });
                                    });
                                },
                                _ => {},
                            }
                        }
                    });
                } 
            },
            _ => {
                let flat = (members, attributes.join("."), kind.to_string());
                self.flats.push(flat);
            },
        }
    }
}
