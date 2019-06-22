use std::collections::HashSet;
use crate::core::{
    Assignment,
    Variable,
    Dictionary,
    Walker,
};

pub struct Parameter {
    assignments: Vec<Assignment>,
    variables: HashSet<Variable>,
}

impl Parameter {
    pub fn get_assignments(&self) -> &Vec<Assignment> {
        &self.assignments
    }

    pub fn get_variables(&self) -> &HashSet<Variable> {
        &self.variables
    }

    pub fn parse(walker: &Walker, dict: &Dictionary) -> Vec<Parameter> {
        let mut parameters = vec![];
        if walker.node.name == "FunctionCall" {
            for walker in walker.direct_childs(|_| true) {
                parameters.push(Parameter::parse_one(&walker, dict));
            }
        }
        parameters
    }

    pub fn parse_one(walker: &Walker, dict: &Dictionary) -> Parameter {
        Parameter {
            assignments: Assignment::parse(&walker, dict),
            variables: Variable::parse(&walker, dict),
        }
    }
}
