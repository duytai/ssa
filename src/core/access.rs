use std::collections::HashSet;
use crate::core::{
    Assignment,
    Variable,
    Dictionary,
    Walker,
};

pub struct IndexAccess {
    assignments: Vec<Assignment>,
    variables: HashSet<Variable>,
}

impl IndexAccess {
    pub fn get_assignments(&self) -> &Vec<Assignment> {
        &self.assignments
    }

    pub fn get_variables(&self) -> &HashSet<Variable> {
        &self.variables
    }

    pub fn parse(walker: &Walker, dict: &Dictionary) -> Vec<IndexAccess> {
        let mut accesses = vec![];
        let fi = |walker: &Walker, _: &Vec<Walker>| {
            walker.node.name == "IndexAccess"
        };
        let ig = |walker: &Walker, _: &Vec<Walker>| {
            walker.node.name == "FunctionCall"
        };
        for walker in walker.walk(false, ig, fi) {
            for walker in walker.direct_childs(|_| true) {
                accesses.push(IndexAccess::parse_one(&walker, dict));
            }
        }
        accesses
    }

    pub fn parse_one(walker: &Walker, dict: &Dictionary) -> IndexAccess {
        IndexAccess {
            assignments: Assignment::parse(&walker, dict),
            variables: Variable::parse(&walker, dict),
        }
    }
}
