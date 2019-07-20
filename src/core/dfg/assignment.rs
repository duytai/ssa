use std::collections::HashSet;
use crate::core::{
    Variable,
    Walker,
    Dictionary,
    Operator,
};


/// The statement edits the flow of data in a solidity program 
///
/// The value of a variable can be passed to other variables through assignment statement or
/// combined with other variables. The assignment not only standard expression (LHS = RHS) but also LHS without RHS. The
/// full list of solidity tokens possibly including the assignment statement:
///
/// 1. __Assignment__ : a standard assignment inside body of a function.
///
/// ```javacript
/// x = y + 20;
/// ```
///
/// 2. __VariableDeclaration__: a state variable declaration in a contract
///
/// ```javacript
/// contract Sample {
///   uint totalSupply = 0;
/// }
/// ```
///
/// 3. __ParameterList__: a list of parameters of a function
///
/// ```javascript
/// contract Sample {
///   function add(uint x, uint y) returns(uint) {}
/// }
/// ```
/// 4. __VariableDeclarationStatement__: local variable declaration
///
/// ```javascript
/// uint x = y + 10;
/// ```

#[derive(Debug, Clone)]
pub struct Assignment {
    /// a list of variables in LHS of a assignment
    lhs: HashSet<Variable>,
    /// a list of variables in RHS of a assignment
    rhs: HashSet<Variable>,
    /// the operator in a assignment
    op: Operator,
}

impl Assignment {

    pub fn new(lhs: HashSet<Variable>, rhs: HashSet<Variable>, op: Operator) -> Self {
        Assignment { lhs, rhs, op }
    }

    pub fn get_lhs(&self) -> &HashSet<Variable> {
        &self.lhs
    }

    pub fn get_rhs(&self) -> &HashSet<Variable> {
        &self.rhs
    }

    pub fn get_op(&self) -> &Operator {
        &self.op
    }

    /// Find all variables in current walker, the dictionary is used to identify global variables 
    pub fn parse(walker: &Walker, dict: &Dictionary) -> Vec<Assignment> {
        let mut assignments = vec![];
        let fi = |walker: &Walker, _: &Vec<Walker>| {
            let operator = walker.node.attributes["operator"].as_str().unwrap_or("");
            walker.node.name == "Assignment"
            || operator == "++"
            || operator == "--"
            || operator == "delete"
        };
        let ig = |walker: &Walker, _: &Vec<Walker>| {
            walker.node.name == "FunctionCall"
            || walker.node.name == "ModifierInvocation"
            || walker.node.name == "VariableDeclaration"
            || walker.node.name == "VariableDeclarationStatement"
            || walker.node.name == "MemberAccess"
            || walker.node.name == "Identifier"
            || walker.node.name == "IndexAccess"
        };
        for walker in walker.walk(false, ig, fi).into_iter() {
            assignments.push(Assignment::parse_one(&walker, dict));
        }
        assignments
    }

    /// Find a assignment of current walker
    fn parse_one(walker: &Walker, dict: &Dictionary) -> Assignment {
        let operator = walker.node.attributes["operator"].as_str();
        let op = match operator {
            Some(op) => match op {
                "=" | "delete" => Operator::Equal,
                _ => Operator::Other,
            },
            None => Operator::Equal, 
        };
        let mut lhs = HashSet::new();
        let mut rhs = HashSet::new();
        let walkers = walker.direct_childs(|_| true);
        lhs.extend(Variable::parse(&walkers[0], dict));
        if walkers.len() >= 2 {
            rhs.extend(Variable::parse(&walkers[1], dict));
        }
        Assignment { lhs, rhs, op }
    }
}
