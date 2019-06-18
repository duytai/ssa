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

    /// Export data to tuple format
    pub fn to_tuple(&self) -> (u32, u32, &Variable) {
        (self.from, self.to, &self.var)
    }
}
