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
        // Data location: memory, storage, calldata
        // Origin: pointer, ref 
        walker.node.attributes["type"].as_str().map(|type_str| {
            let mut norm_type = type_str.to_string();
            for keyword in vec!["memory", "storage", "calldata", "pointer", "ref"] {
                let temp = norm_type.clone();
                norm_type.clear();
                for item in temp.split(keyword) {
                    norm_type.push_str(item.trim());
                }
            }
            norm_type
        })
    } 

    /// Find all variables of the walker, we need the dictionary to identify `Member::Global`
    ///
    /// Ignore node and its childs if it is listed in visited_nodes
    pub fn parse(walker: &Walker, dict: &Dictionary) -> HashSet<Self> {
        let mut ret = HashSet::new();
        let fi = |walker: &Walker, _: &Vec<Walker>| {
            walker.node.name == "MemberAccess"
            || walker.node.name == "Identifier"
            || walker.node.name == "IndexAccess"
        };
        let ig = |walker: &Walker, _: &Vec<Walker>| {
            walker.node.name == "VariableDeclaration"
            || walker.node.name == "VariableDeclarationStatement"
            || walker.node.name == "Assignment"
            || walker.node.name == "FunctionCall"
        };
        for walker in walker.walk(true, ig, fi) {
            Variable::parse_one(&walker, dict).map(|variable| {
                ret.insert(variable);
            });
        }
        ret
    }

    fn parse_one(walker: &Walker, dict: &Dictionary) -> Option<Self> {
        let members = Variable::find_members(walker, dict);
        if !members.is_empty() {
            let variable = Variable {
                members,
                source: walker.node.source.to_string(),
                kind: Variable::normalize_type(walker),
            };
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
        let attributes = walker.node.attributes;
        let reference = attributes["referencedDeclaration"].as_u32();
        let prop = (
            reference,
            reference.and_then(|reference| dict.walker_at(reference)),
            attributes["member_name"].as_str().unwrap_or(""),
            attributes["value"].as_str().unwrap_or(""),
        );
        match walker.node.name {
            "Identifier" => {
                let mut ret = vec![];
                match prop {
                    (Some(reference), walker, _, value) => {
                        match walker {
                            Some(_) => ret.push(Member::Reference(reference)),
                            None => ret.push(Member::Global(value.to_string())),
                        }
                    },
                    (None, _, member_name, _) => ret.push(Member::Global(member_name.to_string())),
                }
                ret
            },
            "MemberAccess" => {
                let mut ret = vec![];
                match prop {
                    (Some(reference), walker, member_name, _) => {
                        match walker {
                            Some(_) => ret.push(Member::Reference(reference)),
                            None => ret.push(Member::Global(member_name.to_string())),
                        }
                    },
                    (None, _, member_name, _) => ret.push(Member::Global(member_name.to_string())),
                }
                for walker in walker.direct_childs(|_| true).into_iter() {
                    ret.append(&mut Variable::find_members(&walker, dict));
                }
                ret
            },
            "IndexAccess" => {
                let mut ret = vec![];
                let walkers = walker.direct_childs(|_| true);
                walkers.get(1).map(|_| ret.push(Member::IndexAccess));
                walkers.get(0).map(|walker| ret.append(&mut Variable::find_members(&walker, dict)));
                ret
            },
            _ => vec![],
        }
    }
}
