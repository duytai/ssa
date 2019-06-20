use std::collections::HashSet;
use crate::core::{
    Variable,
    Walker,
    Dictionary,
};

/// Operator in an assignment statement
///
/// - `Operator::Equal` : the variable in LHS clears it own value and create a data dependency on all variables in RHS
/// ```javascript
/// x = y;
/// KILL(x), USE(Y)
/// ```javascript
/// - `Operator::Other` : the variable in LHS is modified by using both its value and
/// RHS
/// ```javascript
/// x += y;
/// USE(x), USE(y)
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    /// Operator =
    Equal,
    /// Other operators: |=, ^=, &=, <<=, >>=, +=, -=, *=, /=, %=
    Other,
}

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

#[derive(Debug)]
pub struct Assignment {
    /// a list of variables in LHS of a assignment
    lhs: HashSet<Variable>,
    /// a list of variables in RHS of a assignment
    rhs: HashSet<Variable>,
    /// the operator in a assignment
    op: Operator,
}

impl Assignment {

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
    ///
    /// Ignore node and its childs if it is listed in visited_nodes
    pub fn parse(walker: &Walker, dict: &Dictionary, visited_nodes: &mut HashSet<u32>) -> Vec<Assignment> {
        let mut assignments = vec![];
        let mut new_visted_nodes = HashSet::new();
        let fi = |walker: &Walker| {
            let operator = walker.node.attributes["operator"].as_str().unwrap_or("");
            visited_nodes.contains(&walker.node.id)
            || walker.node.name == "VariableDeclaration"
            || walker.node.name == "VariableDeclarationStatement"
            || walker.node.name == "Assignment"
            || operator == "++"
            || operator == "--"
            || operator == "delete"
        };
        for walker in walker.all_childs(true, fi).into_iter() {
            if !visited_nodes.contains(&walker.node.id) {
                assignments.push(Assignment::parse_one(&walker, dict));
                new_visted_nodes.insert(walker.node.id);
            }
        }
        visited_nodes.extend(new_visted_nodes);
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
        if walker.node.name == "VariableDeclaration" {
            lhs.extend(Variable::parse(walker, dict, &mut HashSet::new()));
        } else {
            lhs.extend(Variable::parse(&walkers[0], dict, &mut HashSet::new()));
        }
        if walkers.len() >= 2 {
            rhs.extend(Variable::parse(&walkers[1], dict, &mut HashSet::new()));
        }
        Assignment { lhs, rhs, op }
    }
}
