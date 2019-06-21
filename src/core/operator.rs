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
