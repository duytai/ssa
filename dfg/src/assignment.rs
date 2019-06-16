use std::collections::HashSet;
use crate::variable::{ Variable };
use crate::core::{ Walker, Dictionary };

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Equal,
    Other,
}

#[derive(Debug)]
pub struct Assignment {
    lhs: HashSet<Variable>,
    rhs: HashSet<Variable>,
    op: Operator,
}

impl Assignment {
    pub fn to_tuple(&self) -> (&HashSet<Variable>, &HashSet<Variable>, &Operator) {
        (&self.lhs, &self.rhs, &self.op)
    }

    pub fn parse(walker: &Walker, dict: &Dictionary) -> Vec<Assignment> {
        let mut assignments = vec![];
        match walker.node.name {
            // state variables
            "VariableDeclaration" => {
                let op = Operator::Equal;
                let lhs = Variable::parse(&walker, dict);
                let rhs = HashSet::new();
                assignments.push(Assignment { lhs, rhs, op });
            },
            // variables from parameters
            "ParameterList" => {
                walker.for_each(|walker, _| {
                    let op = Operator::Equal;
                    let lhs = Variable::parse(&walker, dict);
                    let rhs = HashSet::new();
                    assignments.push(Assignment { lhs, rhs, op });
                });
            },
            // local variable definitions
            "VariableDeclarationStatement" => {
                assignments.push(Assignment::parse_one(&walker, dict));
            },
            // variable assignments
            _ => {
                walker.for_all(|walker| {
                    walker.node.name == "Assignment"
                }, |walkers| {
                    for walker in walkers {
                        assignments.push(Assignment::parse_one(&walker, dict));
                    }
                });
            },
        }
        assignments
    }

    fn parse_one(walker: &Walker, dict: &Dictionary) -> Assignment {
        let operator = walker.node.attributes["operator"].as_str().unwrap_or("=");
        let op = match operator {
            "=" => Operator::Equal,
            _ => Operator::Other, 
        };
        let mut lhs = HashSet::new();
        let mut rhs = HashSet::new();
        walker.for_all(|_| { true }, |walkers| {
            lhs.extend(Variable::parse(&walkers[0], dict));
            if walkers.len() >= 2 {
                rhs.extend(Variable::parse(&walkers[1], dict));
            }
        });
        Assignment { lhs, rhs, op }
    }
}
