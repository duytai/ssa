use std::collections::HashSet;
use super::variable::Variable;
use super::assignment::Assignment;

#[derive(Debug)]
pub struct FlowTable {
    variables: HashSet<Variable>
}

#[derive(Debug)]
pub enum FlowItem {
    Variables(HashSet<Variable>),
    Assignments(Vec<Assignment>),
}

impl FlowTable {
    pub fn new() -> Self {
        FlowTable { variables: HashSet::new() }
    }

    pub fn insert(&mut self, item: FlowItem) {
        match item {
            FlowItem::Variables(variables) => {
                for variable in variables {
                    let mut variable = variable;
                    if self.variables.contains(&variable) {
                        let mut v = self.variables.get(&variable).unwrap();
                        variable.kill = variable.kill && v.kill;
                    }
                    self.variables.insert(variable);
                }
            },
            FlowItem::Assignments(assignments) => {
            }
        }
    }
}
