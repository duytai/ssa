use std::collections::HashSet;
use super::variable::{ Variable };
use crate::walker::{ Walker };
use crate::dict::{ Dictionary };

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
    pub fn parse(walker: &Walker, dict: &Dictionary) -> Vec<Assignment> {
        let mut assignments = vec![];
        match walker.node.name {
            "VariableDeclarationStatement" => {
                if let Some(assignment) = Assignment::parse_one(walker, dict) {
                    assignments.push(assignment);
                }
            },
            _ => {
                walker.for_all(|walker| {
                    walker.node.name == "Assignment"
                }, |walkers| {
                    for walker in walkers {
                        if let Some(assignment) = Assignment::parse_one(&walker, dict) {
                            assignments.push(assignment);
                        }
                    }
                });
            },
        }
        assignments
    }

    fn parse_one(walker: &Walker, dict: &Dictionary) -> Option<Assignment> {
        let operator = walker.node.attributes["operator"].as_str().unwrap_or("=");
        let op = match operator {
            "=" => Operator::Equal,
            _ => Operator::Other, 
        };
        let mut lhs = HashSet::new();
        let mut rhs = HashSet::new();
        let mut has_rhs = false;
        walker.for_all(|_| { true }, |walkers| {
            lhs.extend(Variable::parse(&walkers[0], dict));
            if walkers.len() >= 2 {
                rhs.extend(Variable::parse(&walkers[1], dict));
                has_rhs = true;
            }
        });
        if has_rhs {
            Some(Assignment { lhs, rhs, op })
        } else {
            None
        }
    }
}
