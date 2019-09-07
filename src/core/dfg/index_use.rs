use std::collections::HashSet;
use crate::core::{
    Assignment,
    Member,
    Variable,
    Dictionary,
    Walker,
    Operator,
};

pub struct IndexUse {
    assignments: Vec<Assignment>,
    variables: HashSet<Variable>,
}

impl IndexUse {
    pub fn get_assignments(&self) -> &Vec<Assignment> {
        &self.assignments
    }

    pub fn get_variables(&self) -> &HashSet<Variable> {
        &self.variables
    }

    pub fn parse(walker: &Walker, dict: &Dictionary) -> Vec<IndexUse> {
        let mut index_use = IndexUse {
            assignments: vec![],
            variables: HashSet::new(),
        };
        let ig = |_: &Walker, _: &Vec<Walker>| false;
        let fi = |walker: &Walker, _: &Vec<Walker>| {
            walker.node.name == "IndexAccess"
        };
        if walker.node.name == "IndexAccess" {
            let mut lhs = HashSet::new();
            let rhs = HashSet::new();
            let op = Operator::Equal;
            lhs.insert(IndexUse::to_var(walker, dict));
            index_use.assignments.push(Assignment::new(lhs, rhs, op));
        } else {
            for walker in walker.walk(true, ig, fi).into_iter() {
                index_use.variables.insert(IndexUse::to_var(&walker, dict));
            }
        }
        vec![index_use]
    }

    pub fn to_var(walker: &Walker, dict: &Dictionary) -> Variable {
        let members = IndexUse::find_members(walker, dict);
        members.last()
            .and_then(|member| match member {
                Member::Reference(id) => dict.lookup(*id),
                _ => None,
            })
            .map(|walker| {
                let variable = Variable::new(
                    vec![Member::Reference(walker.node.id)],
                    walker.node.source.to_string(),
                    Variable::normalize_type(walker),
                );
                for variable in variable.flatten(dict) {
                    println!("variable: {:?}", variable);
                }
            });
        Variable::new(
            vec![Member::Global(walker.node.id.to_string())],
            walker.node.source.to_string(),
            Variable::normalize_type(walker)
        )
    }

    fn find_members(walker: &Walker, dict: &Dictionary) -> Vec<Member> {
        let attributes = walker.node.attributes;
        let reference = attributes["referencedDeclaration"].as_u32();
        let prop = (
            reference,
            reference.and_then(|reference| dict.lookup(reference)),
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
                    ret.append(&mut IndexUse::find_members(&walker, dict));
                }
                ret
            },
            "IndexAccess" => {
                let mut ret = vec![];
                let walkers = walker.direct_childs(|_| true);
                walkers.get(1).map(|_| ret.push(Member::IndexAccess));
                walkers.get(0).map(|walker| ret.append(&mut IndexUse::find_members(&walker, dict)));
                ret
            },
            _ => vec![],
        }
    }
}
