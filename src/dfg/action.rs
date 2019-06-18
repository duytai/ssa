use crate::dfg::Variable;

/// The behaviour of a variable
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    // A variable is used somewhere
    Use(Variable, u32),
    // A variable is completely cleared and store new data 
    Kill(Variable, u32),
}

