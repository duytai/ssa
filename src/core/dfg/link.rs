use crate::core::Variable;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DataLinkLabel {
    Internal, // Inside current function
}

/// Data dependency link between to node
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DataLink {
    from: (Variable, u32),
    to: (Variable, u32),
    label: DataLinkLabel,
}

impl DataLink {
    pub fn new(from: (Variable, u32), to: (Variable, u32), label: DataLinkLabel) -> Self {
        DataLink { from, to, label }
    }

    pub fn get_from(&self) -> &(Variable, u32) {
        &self.from
    }

    pub fn get_to(&self) -> &(Variable, u32) {
        &self.to
    }

    pub fn get_label(&self) -> &DataLinkLabel {
        &self.label
    }
}
