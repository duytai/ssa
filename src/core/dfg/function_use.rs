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
            lhs.insert(FunctionUse::to_var(walker, dict));
            function_use.assignments.push(Assignment::new(lhs, rhs, op));
        } else {
            for walker in walker.walk(true, ig, fi).into_iter() {
                function_use.variables.insert(FunctionUse::to_var(&walker, dict));
            }
        }
        vec![function_use]
    }

    pub fn to_var(walker: &Walker, dict: &Dictionary) -> Variable {
        let source = walker.node.source;
        let mut variable = Variable::new(
            vec![Member::Reference(walker.node.id)],
            walker.node.source.to_string(),
            Variable::normalize_type(walker)
        );
        let walker = walker.direct_childs(|_| true).get(0)
            .and_then(|walker| walker.node.attributes["referencedDeclaration"].as_u32())
            .and_then(|reference| dict.lookup(reference))
            .and_then(|walker| walker.direct_childs(|_| true).get(1).map(|x| x.clone()))
            .and_then(|walker| match walker.node.name {
                "ParameterList" => walker.direct_childs(|_| true).get(0).map(|x| x.clone()),
                _ => None,
            })
            .and_then(|walker| Some(walker));
        if let Some(walker) = &walker {
            variable = Variable::new(
                vec![Member::Reference(walker.node.id)],
                source.to_string(),
                Variable::normalize_type(walker)
            );
        }
        variable
    }
}
