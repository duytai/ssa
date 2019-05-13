use std::collections::HashSet;
use super::variable::Variable;

#[derive(Debug)]
pub struct FlowTable {
    variables: HashSet<Variable>
}

impl FlowTable {
    pub fn new() -> Self {
        FlowTable { variables: HashSet::new() }
    }

    pub fn insert_variables(&mut self, variables: HashSet<Variable>) {
        for variable in variables {
        }
    }
}
