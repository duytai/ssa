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

    pub fn parse(walker: &Walker, _: &Dictionary) -> Vec<FunctionUse> {
        let mut function_use = FunctionUse {
            assignments: vec![],
            variables: HashSet::new(),
        };
        if walker.node.name == "FunctionCall" {
            let mut lhs = HashSet::new();
            let rhs = HashSet::new();
            let op = Operator::Equal;
            lhs.insert(FunctionUse::to_var(walker));
            function_use.assignments.push(Assignment::new(lhs, rhs, op));
        } else {
            let ig = |_: &Walker, _: &Vec<Walker>| false;
            let fi = |walker: &Walker, _: &Vec<Walker>| {
                walker.node.name == "FunctionCall"
            };
            for walker in walker.walk(true, ig, fi).into_iter() {
                function_use.variables.insert(FunctionUse::to_var(&walker));
            }
        }
        vec![function_use]
    }

    pub fn to_var(walker: &Walker) -> Variable {
        let members = vec![Member::Reference(walker.node.id)];
        let mut source: Option<&str> = None;
        for (index, walker) in walker.direct_childs(|_| true).into_iter().enumerate() {
            if index == 0 {
                source = Some(walker.node.source);
            }
        }
        Variable::new(members, source.unwrap_or("").to_string())
    }
}
