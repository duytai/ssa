use std::{
    collections::{ HashMap, HashSet },
};
use super::{
    variable::{ Variable, VariableComparison },
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
                for Assignment { lhs, rhs, op } in assignments {
                    for variable in lhs.iter() {
                        for (v, kill) in child.variables.iter() {
                            if !kill {
                                match v.contains(&variable) {
                                    VariableComparison::Equal => {
                                        if op == Operator::Equal {
                                            table.variables.insert(v.clone(), true);
                                        }
                                        for r in rhs.iter() {
                                            table.variables.insert(r.clone(), false);
                                        }
                                    },
                                    VariableComparison::Partial => {
                                        if op == Operator::Equal {
                                            table.variables.insert(v.clone(), true);
                                        }
                                        for r in rhs.iter() {
                                            let new_variable = v.clone().merge(r);
                                            table.variables.insert(new_variable, false);
                                        }
                                    },
                                    VariableComparison::NotEqual => {},
                                }
                            }
                        }
                    }
                } 
            }, 
            FlowItem::Comparison => {
            },
            FlowItem::None => {
            },
        }
        table
    }
}
