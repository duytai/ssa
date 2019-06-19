use crate::dfg::Variable;

/// Data dependency link between to node
///
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DataLink {
    /// Start at from
    from: u32,
    /// Stop at to
    to: u32,
    /// What variable that link describes
    var: Variable,
}

impl DataLink {
    /// Simply create a link
    pub fn new(from: u32, to: u32, var: Variable) -> Self {
        DataLink { from, to, var }
    }

    fn get_from(&self) -> u32 {
        self.from
    }

    fn get_to(&self) -> u32 {
        self.to
    }

    fn get_var(&self) -> &Variable {
        &self.var
    }
}
