use std::collections::HashSet;
use crate::core::{
    Walker,
    Dictionary,
    Member,
    VariableComparison,
    FlatVariable,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Variable {
    members: Vec<Member>,
    source: String,
    kind: String,
}

impl Variable {

    pub fn new(members: Vec<Member>, source: String, kind: String) -> Self {
        Variable { members, source, kind }
    }

    pub fn get_members(&self) -> &Vec<Member> {
        &self.members
    }

    pub fn get_source(&self) -> &str {
        &self.source
    }

    pub fn get_kind(&self) -> &str {
        &self.kind
    }

    pub fn parse(walker: &Walker, dict: &Dictionary) -> HashSet<Self> {
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
        // TODO: add variables to ret
        for walker in walker.walk(true, ig, fi) {
            let flat_variable = FlatVariable::new(&walker, dict);
        }
        HashSet::new()
    }


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

}
