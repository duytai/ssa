/// Edge of CFG
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Edge {
    from: u32,
    to: u32,
}

impl Edge {
    pub fn new(from: u32, to: u32) -> Self {
        Edge { from, to }
    }

    pub fn to_tuple(&self) -> (u32, u32) {
        (self.from, self.to)
    }
}

