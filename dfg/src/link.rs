use crate::variable::Variable;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DataLink {
    pub from: u32,
    pub to: u32,
    pub var: Variable,
}
