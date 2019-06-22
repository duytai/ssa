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

    pub fn get_from(&self) -> u32 {
        self.from
    }

    pub fn get_to(&self) -> u32 {
        self.to
    }
}

