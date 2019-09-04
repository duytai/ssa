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
    from_var: Variable,
    /// Stop at to
    to: u32,
    to_var: Variable,
    /// label
    label: DataLinkLabel,
}

impl DataLink {
    pub fn new(
        from: u32,
        from_var: Variable,
        to: u32,
        to_var: Variable,
        label: DataLinkLabel
    ) -> Self {
        DataLink { from, from_var, to, to_var, label }
    }

    pub fn get_from(&self) -> u32 {
        self.from
    }

    pub fn get_to(&self) -> u32 {
        self.to
    }

    pub fn get_from_var(&self) -> &Variable {
        &self.from_var
    }

    pub fn get_to_var(&self) -> &Variable {
        &self.to_var
    }

    pub fn get_label(&self) -> &DataLinkLabel {
        &self.label
    }
}
