use crate::variable::Variable;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Use(Variable, u32),
    Kill(Variable, u32),
}

