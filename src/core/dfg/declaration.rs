use std::collections::HashSet;
use crate::core::{
    Variable,
    Walker,
    Dictionary,
    Operator,
    Assignment,
    FlatVariable,
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
            match walker.node.name {
                "VariableDeclaration" => {
                    !(path.len() >= 2 && path[path.len() - 2].node.name == "VariableDeclarationStatement")
                },
                "VariableDeclarationStatement" => true,
                _ => false,
            }
        };
        let ig = |walker: &Walker, _: &Vec<Walker>| {
            walker.node.name == "FunctionCall"
            || walker.node.name == "ModifierInvocation"
            || walker.node.name == "MemberAccess"
            || walker.node.name == "Identifier"
            || walker.node.name == "IndexAccess"
            || walker.node.name == "Assignment"
        };
        for walker in walker.walk(false, ig, fi).into_iter() {
            let op = Operator::Equal;
            let walkers = walker.direct_childs(|_| true);
            let flat_variable = FlatVariable::new(&walker, dict);
            let lhs = flat_variable.get_variables();
            let mut rhs = HashSet::new();
            if walkers.len() >= 2 {
                rhs.extend(Variable::parse(&walkers[1], dict));
            }
            let assignment = Assignment::new(lhs, rhs, op);
            declarations.push(Declaration { assignment });
        }
        declarations
    }
}
