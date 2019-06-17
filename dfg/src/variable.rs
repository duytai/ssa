use std::collections::HashSet;
use crate::core::{ Walker, Dictionary };

/// Variable access
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Member {
    /// Link to node that defines current variable
    Reference(u32),
    /// Contains name of a global variable or function
    Global(String),
    /// Accesses a member in an array
    IndexAccess,
}

/// Relationship between two variables
#[derive(Debug, PartialEq, Eq)]
pub enum VariableComparison {
    /// Completely the same
    Equal,
    /// Completely different
    NotEqual,
    /// One variable contains other variable
    Partial,
}

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
    /// Export data to tuple format
    pub fn to_tuple(&self) -> (&Vec<Member>, &String) {
        (&self.members, &self.source)
    }

    /// Find all variables of the walker, we need the dictionary to identify `Member::Global`
    /// member
    pub fn parse(walker: &Walker, dict: &Dictionary) -> HashSet<Self> {
        let mut ret = HashSet::new();
        let variable = Variable::parse_one(&walker, dict);
        if walker.node.name != "FunctionCall" {
            if variable.is_some() {
                ret.insert(variable.unwrap());
            } else {
                walker.all_break(|walker| {
                    walker.node.name == "FunctionCall"
                    || walker.node.name == "Identifier"
                    || walker.node.name == "MemberAccess"
                    || walker.node.name == "IndexAccess"
                }, |walkers| {
                    for walker in walkers {
                        if walker.node.name != "FunctionCall" {
                            let variable = Variable::parse_one(&walker, dict);
                            if variable.is_some() {
                                ret.insert(variable.unwrap());
                            }
                        }
                    }
                });
            }
        }
        ret
    }

    fn parse_one(walker: &Walker, dict: &Dictionary) -> Option<Self> {
        let mut variable = Variable {
            members: vec![],
            source: walker.node.source.to_string(),
        };
        variable.members = Variable::find_variable(walker, dict);
        if variable.members.len() > 0 {
            Some(variable)
        } else {
            None
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
                        false => VariableComparison::NotEqual,
                    }
                },
                false => VariableComparison::NotEqual,
            }
        }
    }

    fn find_variable(walker: &Walker, dict: &Dictionary) -> Vec<Member> {
        let reference = walker.node.attributes["referencedDeclaration"].as_u32();
        let member_name = walker.node.attributes["member_name"].as_str().unwrap_or("");
        let value = walker.node.attributes["value"].as_str().unwrap_or("");
        match walker.node.name {
            "VariableDeclaration" => {
                vec![Member::Reference(walker.node.id)]
            },
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
                walker.for_each(|walker, _| {
                    ret.append(&mut Variable::find_variable(&walker, dict));
                });
                ret
            },
            "IndexAccess" => {
                let mut ret = vec![];
                walker.for_each(|walker, index| {
                    if index == 0 {
                        ret.append(&mut Variable::find_variable(&walker, dict));
                    } else if index == 1 {
                        ret.insert(0, Member::IndexAccess);
                    }
                });
                ret
            },
            _ => vec![],
        }
    }
}
