use std::collections::HashSet;
use crate::core::{
    Walker,
    Dictionary,
};

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

    pub fn get_members(&self) -> &Vec<Member> {
        &self.members
    }

    pub fn get_source(&self) -> &str {
        &self.source
    }

    /// Find all variables of the walker, we need the dictionary to identify `Member::Global`
    ///
    /// Ignore node and its childs if it is listed in visited_nodes
    pub fn parse(walker: &Walker, dict: &Dictionary, visited_nodes: &mut HashSet<u32>) -> HashSet<Self> {
        let mut ret = HashSet::new();
        let mut new_visited_nodes = HashSet::new();
        let mut index_acceses = HashSet::new(); 
        let fi = |walker: &Walker| {
            visited_nodes.contains(&walker.node.id)
            || walker.node.name == "FunctionCall"
            || walker.node.name == "Identifier"
            || walker.node.name == "MemberAccess"
            || walker.node.name == "IndexAccess"
            || walker.node.name == "VariableDeclaration"
        };
        for walker in walker.all_childs(true, fi) {
            if walker.node.name != "FunctionCall" && !visited_nodes.contains(&walker.node.id) {
                Variable::parse_one(&walker, dict, &mut index_acceses).map(|variable| {
                    ret.insert(variable);
                });
                new_visited_nodes.insert(walker.node.id);
            }
        }
        for index_access in index_acceses {
            dict.lookup(index_access).map(|walker| {
                ret.extend(Variable::parse(&walker, dict, &mut HashSet::new()));
            });
        }
        visited_nodes.extend(new_visited_nodes);
        ret
    }

    fn parse_one(walker: &Walker, dict: &Dictionary, index_acceses: &mut HashSet<u32>) -> Option<Self> {
        let mut variable = Variable {
            members: vec![],
            source: walker.node.source.to_string(),
        };
        variable.members = Variable::find_members(walker, dict, index_acceses);
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
                        false => VariableComparison::NotEqual,
                    }
                },
                false => VariableComparison::NotEqual,
            }
        }
    }

    /// Find members of a variable
    ///
    /// A member is reference to place where it is declared , global index, index access of array 
    fn find_members(walker: &Walker, dict: &Dictionary, index_acceses: &mut HashSet<u32>) -> Vec<Member> {
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
                for walker in walker.direct_childs(|_| true).into_iter() {
                    ret.append(&mut Variable::find_members(&walker, dict, index_acceses));
                }
                ret
            },
            "IndexAccess" => {
                let mut ret = vec![];
                for (index, walker) in walker.direct_childs(|_| true).into_iter().enumerate() {
                    if index == 0 {
                        ret.append(&mut Variable::find_members(&walker, dict, index_acceses));
                    } else if index == 1 {
                        index_acceses.insert(walker.node.id);
                        ret.insert(0, Member::IndexAccess);
                    }
                }
                ret
            },
            _ => vec![],
        }
    }
}
