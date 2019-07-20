use std::collections::HashSet;
use crate::core::{
    Assignment,
    Member,
    Variable,
    Dictionary,
    Walker,
    Operator,
};

pub struct FunctionUse {
    assignments: Vec<Assignment>,
    variables: HashSet<Variable>,
}

impl FunctionUse {
    pub fn get_assignments(&self) -> &Vec<Assignment> {
        &self.assignments
    }

    pub fn get_variables(&self) -> &HashSet<Variable> {
        &self.variables
    }

    pub fn parse(walker: &Walker, dict: &Dictionary) -> Vec<FunctionUse> {
        let mut function_use = FunctionUse {
            assignments: vec![],
            variables: HashSet::new(),
        };
        let ig = |_: &Walker, _: &Vec<Walker>| false;
        let fi = |walker: &Walker, _: &Vec<Walker>| {
            walker.node.name == "FunctionCall"
        };
        if walker.node.name == "FunctionCall" {
            let mut lhs = HashSet::new();
            let rhs = HashSet::new();
            let op = Operator::Equal;
            lhs.insert(FunctionUse::to_var(walker));
            function_use.assignments.push(Assignment::new(lhs, rhs, op));
        } else {
            for walker in walker.walk(true, ig, fi).into_iter() {
                function_use.variables.insert(FunctionUse::to_var(&walker));
            }
        }
        vec![function_use]
    }

    pub fn to_var(walker: &Walker) -> Variable {
        let members = vec![Member::Reference(walker.node.id)];
        let source = walker.node.source;
        Variable::new(members, source.to_string())
    }
}
