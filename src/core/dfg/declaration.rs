use std::collections::HashSet;
use crate::core::{
    Variable,
    Walker,
    Dictionary,
    Operator,
    Member,
    Assignment,
};

#[derive(Debug)]
pub struct Declaration {
    assignment: Assignment,
}

impl Declaration {

    pub fn get_assignment(&self) -> &Assignment {
        &self.assignment
    }
    /// Find all variables in current walker, the dictionary is used to identify global variables 
    pub fn parse(walker: &Walker, dict: &Dictionary) -> Vec<Declaration> {
        let mut declarations = vec![];
        let fi = |walker: &Walker, path: &Vec<Walker>| {
            if walker.node.name == "VariableDeclaration" {
                if path.len() >= 2 {
                    let w = &path[path.len() - 2];
                    w.node.name !=  "VariableDeclarationStatement"
                } else {
                    true
                }
            } else {
                walker.node.name == "VariableDeclarationStatement"
            }
        };
        let ig = |walker: &Walker, _: &Vec<Walker>| {
            let operator = walker.node.attributes["operator"].as_str().unwrap_or("");
            walker.node.name == "FunctionCall"
            || walker.node.name == "ModifierInvocation"
            || walker.node.name == "MemberAccess"
            || walker.node.name == "Identifier"
            || walker.node.name == "IndexAccess"
            || walker.node.name == "Assignment"
            || operator == "++"
            || operator == "--"
            || operator == "delete"
        };
        for walker in walker.walk(false, ig, fi).into_iter() {
            declarations.push(Declaration::parse_one(&walker, dict));
        }
        declarations
    }

    // Find a assignment of current walker
    fn parse_one(walker: &Walker, dict: &Dictionary) -> Declaration {
        let op = Operator::Equal;
        let mut lhs = HashSet::new();
        let mut rhs = HashSet::new();
        let walkers = walker.direct_childs(|_| true);
        if walker.node.name == "VariableDeclaration" {
            let members = vec![Member::Reference(walker.node.id)];
            let source = walker.node.source.to_string();
            let variable = Variable::new(members, source);
            lhs.insert(variable);
        } else {
            let members = vec![Member::Reference(walkers[0].node.id)];
            let source = walkers[0].node.source.to_string();
            let variable = Variable::new(members, source);
            lhs.insert(variable);
        }
        if walkers.len() >= 2 {
            rhs.extend(Variable::parse(&walkers[1], dict));
        }
        let assignment = Assignment::new(lhs, rhs, op);
        Declaration { assignment }
    }
}
