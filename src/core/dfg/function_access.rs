use std::collections::HashSet;
use crate::core::{
    Assignment,
    Variable,
    Dictionary,
    Walker,
};

pub struct FunctionAccess {
    assignments: Vec<Assignment>,
    variables: HashSet<Variable>,
}

impl FunctionAccess {
    pub fn get_assignments(&self) -> &Vec<Assignment> {
        &self.assignments
    }

    pub fn get_variables(&self) -> &HashSet<Variable> {
        &self.variables
    }

    pub fn parse(walker: &Walker, dict: &Dictionary) -> Vec<FunctionAccess> {
        let mut parameters = vec![];
        if walker.node.name == "FunctionCall" {
            for (index, walker) in walker.direct_childs(|_| true).into_iter().enumerate() {
                if index == 0 {
                    parameters.push(FunctionAccess::parse_one(&walker, dict));
                }
            }
        }
        parameters
    }

    pub fn parse_one(walker: &Walker, dict: &Dictionary) -> FunctionAccess {
        FunctionAccess {
            assignments: Assignment::parse(&walker, dict),
            variables: Variable::parse(&walker, dict),
        }
    }
}
