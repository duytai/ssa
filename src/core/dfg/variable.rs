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
}

impl Variable {

    pub fn new(members: Vec<Member>, source: String) -> Self {
        Variable { members, source }
    }

    pub fn get_members(&self) -> &Vec<Member> {
        &self.members
    }

    pub fn get_source(&self) -> &str {
        &self.source
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
        walker: &Walker,
        dict: &Dictionary,
        mut path: Vec<(Member, String)>,
        paths: &mut Vec<Vec<(Member, String)>>
    ) {
        match walker.node.name {
            "StructDefinition" => {
                let walkers = walker.direct_childs(|_| true);
                for walker in walkers.iter() {
                    self.flatten_variable(walker, dict, path.clone(), paths);
                }
            },
            _ => {
                if walker.node.name == "VariableDeclaration" {
                    let name = walker.node.attributes["name"].as_str().unwrap_or("*");
                    let p = (Member::Reference(walker.node.id), name.to_string());
                    path.push(p);
                }
                let walker = &walker.direct_childs(|_| true)[0];
                match walker.node.name {
                    "UserDefinedTypeName" => {
                        let reference = walker.node.attributes["referencedDeclaration"]
                            .as_u32()
                            .unwrap();
                        let walker = dict.lookup(reference).unwrap();
                        self.flatten_variable(walker, dict, path.clone(), paths);
                    },
                    "ArrayTypeName" => {
                        let p = (Member::IndexAccess, String::from("$"));
                        path.push(p);
                        self.flatten_variable(&walker, dict, path.clone(), paths);
                    },
                    "ElementaryTypeName" => {
                        paths.push(path.clone());
                    },
                    _ => {},
                }
            }
        }
    }

    pub fn flatten(&self, dict: &Dictionary) -> Vec<Variable> {
        self.members.first()
            .and_then(|member| match member {
                Member::Reference(reference) => Some(reference),
                _ => None,
            })
            .and_then(|reference| dict.lookup(*reference))
            .and_then(|walker| {
                let mut flat_variables = vec![];
                let mut paths: Vec<Vec<(Member, String)>> = vec![];
                self.flatten_variable(&walker, dict, vec![], &mut paths);
                for mut path in paths {
                    let mut members = vec![];
                    let mut sources = vec![];
                    path.remove(0);
                    for (member, source) in path {
                        members.insert(0, member);
                        sources.push(source);
                    }
                    members.append(&mut self.members.clone());
                    sources.insert(0, self.source.clone());
                    let variable = Variable::new(members, sources.join("."));
                    flat_variables.push(variable);
                }
                Some(flat_variables)
            })
            .unwrap_or(vec![self.clone()])
    }
}
