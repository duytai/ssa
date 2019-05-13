use crate::walker::{ Walker };

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Member {
    Reference(u32),
    Nothing,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Variable {
    pub members: Vec<Member>,
    pub kill: bool,
}

impl Variable {
    pub fn parse(walker: &Walker) -> Option<Self> {
        let mut variable = Variable { members: vec![], kill: false };
        variable.members = Variable::find_variable(walker);
        if variable.members.len() > 0 {
            Some(variable)
        } else {
            None
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

