use crate::variable::Variable;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DataLink {
    from: u32,
    to: u32,
    var: Variable,
}

impl DataLink {
    pub fn new(from: u32, to: u32, var: Variable) -> Self {
        DataLink { from, to, var }
    }

    pub fn to_tuple(&self) -> (u32, u32, &Variable) {
        (self.from, self.to, &self.var)
    }
}
