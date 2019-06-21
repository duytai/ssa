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
