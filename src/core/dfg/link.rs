use crate::core::Variable;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DataLinkLabel {
    InFrom(u32), // Link to other function
    OutTo(u32), // Exit from current function
    Executor, // this.add() -> this.add
    BuiltIn, // From current function to parameter
    Internal, // Inside current function
}

/// Data dependency link between to node
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DataLink {
    /// Start at from
    from: u32,
    /// Stop at to
    to: u32,
    /// What variable that link describes
    var: Variable,
    /// label
    label: DataLinkLabel,
}

impl DataLink {
    pub fn new(from: u32, to: u32, var: Variable) -> Self {
        DataLink { from, to, var, label: DataLinkLabel::Internal }
    }

    pub fn new_with_label(from: u32, to: u32, var: Variable, label: DataLinkLabel) -> Self {
        DataLink { from, to, var, label }
    }

    pub fn get_from(&self) -> u32 {
        self.from
    }

    pub fn get_to(&self) -> u32 {
        self.to
    }

    pub fn get_var(&self) -> &Variable {
        &self.var
    }

    pub fn get_label(&self) -> &DataLinkLabel {
        &self.label
    }
}
