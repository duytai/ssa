use std::{
    collections::{ HashMap, HashSet },
};
use super::{
    variable::{ Variable, VariableComparison, Member },
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

    pub fn is_vulerable(&self) -> bool {
        let block_timestamp = Variable {
            members: vec![
                Member::Global(String::from("timestamp")),
                Member::Global(String::from("block")),
            ],
        };
        let block_number = Variable {
            members: vec![
                Member::Global(String::from("number")),
                Member::Global(String::from("block")),
            ],
        };
        let now = Variable {
            members: vec![
                Member::Global(String::from("now")),
            ],
        };
        for (variable, _) in self.variables.iter() {
            if variable.contains(&block_timestamp) == VariableComparison::Equal 
               || variable.contains(&now) == VariableComparison::Equal
               || variable.contains(&block_number) == VariableComparison::Equal {
                return true;
            }
        }
        false
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
