use std::collections::HashSet;
use crate::core::{
    Walker,
    Dictionary,
    Member,
    VariableComparison,
};

/// Variable in solidity program
///
/// The variable can be `Array`, `Array Access`, `Struct`, `Struct Access`, `Primitive Type`,
/// `Global Access`. We use the `members` field to describe the different among them.
/// - `Array`, `Struct`, `Primitive`: `members` contains only one `Member::Reference`
/// - `Array Access`: `members` contains one `Member::Reference` and one `Member::IndexAccess`
/// - `Global Access`: `members` will contains at least one `Member::Global`
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Variable {
    members: Vec<Member>,
    source: String,
    kind: Option<String>,
}

impl Variable {

    pub fn new(members: Vec<Member>, source: String, kind: Option<String>) -> Self {
        Variable { members, source, kind }
    }

    pub fn get_members(&self) -> &Vec<Member> {
        &self.members
    }

    pub fn get_source(&self) -> &str {
        &self.source
    }

    pub fn get_type(&self) -> &Option<String> {
        &self.kind
    }

    pub fn normalize_type(walker: &Walker) -> Option<String> {
        walker.node.attributes["type"].as_str().map(|x| x.to_string())
    } 

    /// Find all variables of the walker, we need the dictionary to identify `Member::Global`
    ///
    /// Ignore node and its childs if it is listed in visited_nodes
    pub fn parse(walker: &Walker, dict: &Dictionary) -> HashSet<Self> {
        let mut ret = HashSet::new();
        let fi = |walker: &Walker, _: &Vec<Walker>| {
            walker.node.name == "IndexAccess"
            || walker.node.name == "MemberAccess"
            || walker.node.name == "Identifier"
        };
        let ig = |walker: &Walker, _: &Vec<Walker>| {
            let operator = walker.node.attributes["operator"].as_str().unwrap_or("");
            walker.node.name == "FunctionCall"
            || walker.node.name == "VariableDeclaration"
            || walker.node.name == "VariableDeclarationStatement"
            || walker.node.name == "Assignment"
            || operator == "++"
            || operator == "--"
            || operator == "delete"
        };
        for walker in walker.walk(true, ig, fi) {
            Variable::parse_one(&walker, dict).map(|variable| {
                ret.insert(variable);
            });
        }
        ret
    }

    fn parse_one(walker: &Walker, dict: &Dictionary) -> Option<Self> {
        let mut variable = Variable {
            members: vec![],
            source: walker.node.source.to_string(),
            kind: Variable::normalize_type(walker),
        };
        variable.members = Variable::find_members(walker, dict);
        match variable.members.len() > 0 {
            true => Some(variable),
            false => None,
        }
    }

    /// Use this to find the relationship between two variables
    /// The relationship is:
    /// - `Equal`: if all members fields are the same
    /// - `PartialEq`: if the intersection of two members fields are equal to one of them
    pub fn contains(&self, other: &Variable) -> VariableComparison {
        if other.members.len() > self.members.len() {
            let offset = other.members.len() - self.members.len();
            let sub = &other.members[offset..];
            match sub.iter().eq(self.members.iter()) {
                true => VariableComparison::Partial,
                false => VariableComparison::NotEqual,
            }
        } else {
            let offset = self.members.len() - other.members.len();
            let sub = &self.members[offset..];
            match sub.iter().eq(other.members.iter()) {
                true => {
                    match offset == 0 {
                        true => VariableComparison::Equal,
                        false => VariableComparison::Partial,
                    }
                },
                false => VariableComparison::NotEqual,
            }
        }
    }

    /// Find members of a variable
    ///
    /// A member is reference to place where it is declared , global index, index access of array
    fn find_members(walker: &Walker, dict: &Dictionary) -> Vec<Member> {
        let reference = walker.node.attributes["referencedDeclaration"].as_u32();
        let member_name = walker.node.attributes["member_name"].as_str().unwrap_or("");
        let value = walker.node.attributes["value"].as_str().unwrap_or("");
        match walker.node.name {
            "Identifier" => {
                let mut ret = vec![];
                match reference {
                    Some(reference) => {
                        if dict.lookup(reference).is_some() {
                            ret.push(Member::Reference(reference));
                        } else {
                            ret.push(Member::Global(value.to_string()));
                        }
                    },
                    None => {
                        ret.push(Member::Global(member_name.to_string()));
                    },
                }
                ret
            },
            "MemberAccess" => {
                let mut ret = vec![];
                match reference {
                    Some(reference) => {
                        if dict.lookup(reference).is_some() {
                            ret.push(Member::Reference(reference));
                        } else {
                            ret.push(Member::Global(member_name.to_string()));
                        }
                    },
                    None => {
                        ret.push(Member::Global(member_name.to_string()));
                    }
                }
                for walker in walker.direct_childs(|_| true).into_iter() {
                    ret.append(&mut Variable::find_members(&walker, dict));
                }
                ret
            },
            "IndexAccess" => {
                let mut ret = vec![];
                for (index, walker) in walker.direct_childs(|_| true).into_iter().enumerate() {
                    if index == 0 {
                        ret.append(&mut Variable::find_members(&walker, dict));
                    } else if index == 1 {
                        ret.insert(0, Member::IndexAccess);
                    }
                }
                ret
            },
            _ => vec![],
        }
    }

    fn flatten_variable(
        &self,
        mut walker: Walker,
        dict: &Dictionary,
        mut path: Vec<(Member, String)>,
        paths: &mut Vec<(Vec<(Member, String)>, Option<String>)>
    ) {
        if walker.node.name == "VariableDeclaration" {
            let source = walker.node.attributes["name"].as_str().unwrap_or("*").to_string();
            path.push((Member::Reference(walker.node.id), source));
            loop {
                walker = walker.direct_childs(|_| true)[0].clone();
                let kind = Variable::normalize_type(&walker);
                match walker.node.name {
                    "UserDefinedTypeName" => {
                        let reference = walker.node.attributes["referencedDeclaration"].as_u32().unwrap();
                        let walker = dict.lookup(reference).unwrap();
                        match walker.node.name {
                            "StructDefinition" => {
                                let walkers = walker.direct_childs(|_| true);
                                for walker in walkers {
                                    self.flatten_variable(walker, dict, path.clone(), paths);
                                }
                                break;
                            },
                            "ContractDefinition" => {
                                paths.push((path.clone(), kind));
                                break;
                            },
                            _ => unimplemented!(),
                        }
                    },
                    "ArrayTypeName" => {
                        let source = String::from("$");
                        path.push((Member::IndexAccess, source));
                    },
                    "ElementaryTypeName" => {
                        paths.push((path.clone(), kind));
                        break;
                    },
                    _ => unimplemented!(),
                }
            }
        }
    }

    pub fn flatten(&self, dict: &Dictionary) -> Vec<Variable> {
        let mut flat_variables = vec![];
        if let Some(Member::Reference(reference)) = self.members.first() {
            if let Some(walker) = dict.lookup(*reference) {
                let mut paths = vec![];
                self.flatten_variable(walker.clone(), dict, vec![], &mut paths);
                for (mut path, kind) in paths {
                    let mut members = vec![];
                    let mut sources = vec![];
                    path.remove(0);
                    for (member, source) in path {
                        members.push(member);
                        sources.push(source);
                    }
                    members.reverse();
                    members.append(&mut self.members.clone());
                    sources.insert(0, self.source.clone());
                    let variable = Variable {
                        members,
                        source: sources.join("."),
                        kind,
                    };
                    flat_variables.push(variable);
                }
            }
        }
        if flat_variables.is_empty() {
            flat_variables.push(self.clone());
        }
        flat_variables
    }

    /// Whether a variable can has alias or not
    pub fn can_has_alias(&self, dict: &Dictionary) -> bool {
        let mut members = self.members.clone();
        let mut index_access_count = 0;
        while !members.is_empty() {
            match members.remove(0) {
                Member::Reference(reference) => {
                    if let Some(walker) = dict.lookup(reference) {
                        if walker.node.name == "VariableDeclaration" {
                            let mut walker = walker.clone();
                            while index_access_count >= 0 {
                                walker = walker.direct_childs(|_| true)[0].clone();
                                index_access_count -= 1;
                            }
                            return walker.node.name != "ElementaryTypeName";
                        }
                    }
                    break;
                },
                Member::IndexAccess => {
                    index_access_count += 1;
                },
                _ => break,
            }
        }
        false
    }
}
