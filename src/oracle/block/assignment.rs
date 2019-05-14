use std::collections::HashSet;
use super::variable::{ Variable };

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Equal,
    Other,
}

#[derive(Debug)]
pub struct Assignment {
    pub lhs: HashSet<Variable>,
    pub rhs: HashSet<Variable>,
    pub op: Operator,
}

impl Assignment {
    pub fn new(lhs: HashSet<Variable>, rhs: HashSet<Variable>, op_str: &str) -> Self {
        let mut op = Operator::Equal;
        if op_str != "=" {
            op = Operator::Other;
        }
        Assignment { lhs, rhs, op }
    }
}
