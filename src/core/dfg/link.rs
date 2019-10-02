use crate::core::Variable;

/// Data dependency link between to node
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DataLink {
    from: (Variable, u32),
    to: (Variable, u32),
}

impl DataLink {
    pub fn new(from: (Variable, u32), to: (Variable, u32)) -> Self {
        DataLink { from, to }
    }

    pub fn get_from(&self) -> &(Variable, u32) {
        &self.from
    }

    pub fn get_to(&self) -> &(Variable, u32) {
        &self.to
    }
}
