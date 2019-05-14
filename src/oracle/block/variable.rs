use crate::walker::{ Walker };

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Member {
    Reference(u32),
    Nothing,
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
    pub fn parse(walker: &Walker) -> Option<Self> {
        let mut variable = Variable { members: vec![] };
        variable.members = Variable::find_variable(walker);
        if variable.members.len() > 0 {
            Some(variable)
        } else {
            None
        }
    }

    pub fn contains(&self, other: &Variable) -> VariableComparison {
        let other_len = other.members.len();
        let my_len = self.members.len();
        let sub = &self.members[(my_len - other_len) .. my_len];
        let eq = sub.iter().eq(other.members.iter());
        match eq {
            true => {
                if my_len == other_len {
                    VariableComparison::Equal
                } else {
                    VariableComparison::Partial
                }
            },
            false => VariableComparison::NotEqual,
        }
    }

    fn find_variable(walker: &Walker) -> Vec<Member> {
        match walker.node.name {
            "Identifier" => {
                let reference = walker.node
                    .attributes["referencedDeclaration"]
                    .as_u32()
                    .unwrap();
                vec![Member::Reference(reference)]
            },
            "MemberAccess" => {
                let reference = walker.node
                    .attributes["referencedDeclaration"]
                    .as_u32()
                    .unwrap();
                let mut ret = vec![Member::Reference(reference)];
                walker.for_each(|walker, _| {
                    ret.append(&mut Variable::find_variable(&walker));
                });
                ret
            },
            "IndexAccess" => {
                let mut ret = vec![];
                walker.for_each(|walker, index| {
                    if index == 0 {
                        ret.append(&mut Variable::find_variable(&walker));
                    } else if index == 1 {
                        ret.insert(0, Member::Nothing);
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
    let v1 = Variable { members: vec![Member::Reference(10), Member::Nothing, Member::Reference(20)]};
    let v2 = Variable { members: vec![Member::Nothing, Member::Reference(20)]};
    let v3 = Variable { members: vec![Member::Nothing, Member::Reference(0)]};
    let v4 = Variable { members: vec![Member::Reference(20), Member::Nothing]};
    assert_eq!(v1.contains(&v2), VariableComparison::Partial);
    assert_eq!(v1.contains(&v3), VariableComparison::NotEqual);
    assert_eq!(v1.contains(&v4), VariableComparison::NotEqual);
    assert_eq!(v1.contains(&v1), VariableComparison::Equal);
}
