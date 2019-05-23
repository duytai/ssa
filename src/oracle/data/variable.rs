use std::collections::HashSet;
use crate:: {
    walker::{ Walker },
    dict::{ Dictionary },
};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Member {
    Reference(u32),
    Global(String),
    IndexAccess,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VariableComparison {
    Equal,
    NotEqual,
    Partial,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Variable {
    pub members: Vec<Member>,
}

impl Variable {
    pub fn parse(walker: &Walker, dict: &Dictionary) -> HashSet<Self> {
        let mut ret = HashSet::new();
        let variable = Variable::parse_one(&walker, dict);
        if variable.is_some() {
            ret.insert(variable.unwrap());
        }
        walker.all_break(|walker| {
            walker.node.name == "FunctionCall"
            || walker.node.name == "Identifier"
            || walker.node.name == "MemberAccess"
            || walker.node.name == "IndexAccess"
            || walker.node.name == "Literal"
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
        ret
    }

    fn parse_one(walker: &Walker, dict: &Dictionary) -> Option<Self> {
        let mut variable = Variable { members: vec![] };
        variable.members = Variable::find_variable(walker, dict);
        if variable.members.len() > 0 {
            Some(variable)
        } else {
            None
        }
    }

    // pub fn merge(&self, other: &Variable) -> Variable {
        // let mut members = self.members.clone();
        // if self.members.len() < other.members.len() {
            // panic!("Unsupported merged");
        // }
        // let diff_len = self.members.len() - other.members.len();
        // for i in 0..other.members.len() {
            // members[diff_len + i] = other.members[i].clone();
        // }
        // Variable { members }
    // }

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

#[test]
fn variable_contains() {
    let v1 = Variable { members: vec![Member::Reference(10), Member::IndexAccess, Member::Reference(20)]};
    let v2 = Variable { members: vec![Member::IndexAccess, Member::Reference(20)]};
    let v3 = Variable { members: vec![Member::IndexAccess, Member::Reference(0)]};
    let v4 = Variable { members: vec![Member::Reference(20), Member::IndexAccess]};
    assert_eq!(v1.contains(&v2), VariableComparison::Partial);
    assert_eq!(v1.contains(&v3), VariableComparison::NotEqual);
    assert_eq!(v1.contains(&v4), VariableComparison::NotEqual);
    assert_eq!(v1.contains(&v1), VariableComparison::Equal);
}

#[test]
fn variable_merge() {
    let v1 = Variable {
        members: vec![
            Member::Reference(10),
            Member::IndexAccess,
            Member::Reference(20),
        ],
    };
    let v2 = Variable {
        members: vec![
            Member::Reference(3),
        ],
    };
    let v3 = Variable {
        members: vec![
            Member::Reference(3),
            Member::Reference(3),
            Member::Reference(3),
        ],
    };
    assert_eq!(v1.merge(&v2), Variable { members: vec![
        Member::Reference(10),
        Member::IndexAccess,
        Member::Reference(3),
    ]});
    assert_eq!(v1.merge(&v3), Variable { members: vec![
        Member::Reference(3),
        Member::Reference(3),
        Member::Reference(3),
    ]});
}
