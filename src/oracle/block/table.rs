use std::{
    collections::{ HashMap, HashSet },
};
use super::{
    variable::{ Variable },
    assignment::{ Assignment, Operator },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowTable {
    variables: HashMap<Variable, bool>,
}

#[derive(Debug)]
pub enum FlowItem {
    Variables(HashSet<Variable>),
    Assignments(Vec<Assignment>),
    Comparison,
    None,
}

impl FlowTable {
    pub fn new() -> Self {
        FlowTable {
            variables: HashMap::new(),
        }
    }

    pub fn insert(&mut self, variable: Variable, kill: bool) {
        self.variables.insert(variable, kill);
    }

    pub fn merge(child: &FlowTable, item: FlowItem) -> FlowTable {
        let mut table = child.clone();
        match item {
            FlowItem::Variables(variables) => {
                for variable in variables {
                    table.insert(variable, false);
                }
            },
            FlowItem::Assignments(assignments) => {
            }, 
            FlowItem::Comparison => {
            },
            FlowItem::None => {
            },
        }
        table
    }
}
