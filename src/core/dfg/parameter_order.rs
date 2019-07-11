use std::collections::HashSet;
use crate::core::{
    Assignment,
    Variable,
    Dictionary,
    Walker,
};

pub struct ParameterOrder {
    variables: Vec<HashSet<Variable>>,
}

impl ParameterOrder {
    pub fn get_variables(&self) -> &Vec<HashSet<Variable>> {
        &self.variables
    }

    pub fn parse(walker: &Walker, dict: &Dictionary) -> ParameterOrder {
        let mut po = ParameterOrder { variables: vec![] };
        if walker.node.name == "FunctionCall" {
            for (index, walker) in walker.direct_childs(|_| true).iter().enumerate() {
                // first is attribute of functionCall, others are params
                if index > 0 {
                    let mut variables = Variable::parse(&walker, dict);
                    let assignments = Assignment::parse(&walker, dict);
                    for assignment in assignments {
                        let rhs = assignment.get_rhs().clone();
                        variables.extend(rhs);
                    }
                    po.variables.push(variables);
                }
            }
        }
        po
    }
}
