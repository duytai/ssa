use crate::core::Variable;

/// The behaviour of a variable
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    // A variable is used somewhere
    Use(Variable, u32),
    // A variable is completely cleared and store new data 
    Kill(Variable, u32),
}

/// Relationship between two variables
#[derive(Debug, PartialEq, Eq)]
pub enum VariableComparison {
    /// Completely the same
    Equal,
    /// Completely different
    NotEqual,
    /// One variable contains other variable
    Partial,
}

/// Variable access
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Member {
    /// Link to node that defines current variable
    Reference(u32),
    /// Contains name of a global variable or function
    Global(String),
    /// Accesses a member in an array
    IndexAccess,
    /// Shortcut to other variables
    Shortcut(u32),
}

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
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Operator {
    /// Operator =
    Equal,
    /// Other operators: |=, ^=, &=, <<=, >>=, +=, -=, *=, /=, %=
    Other,
}
