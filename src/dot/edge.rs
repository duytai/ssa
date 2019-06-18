pub enum DotEdge {
    Basic((u32, u32)),
    Labeled((u32, u32, String)),
}

impl From<(u32, u32, String)> for DotEdge {
    fn from(item: (u32, u32, String)) -> Self {
        DotEdge::Labeled(item)
    }
}

impl From<(u32, u32)> for DotEdge {
    fn from(item: (u32, u32)) -> Self {
        DotEdge::Basic(item)
    }
}

impl DotEdge {
    pub fn format(&self) -> String {
        match self {
            DotEdge::Basic(item) => {
                format!("{} -> {};", item.0, item.1)
            },
            DotEdge::Labeled(item) => {
                format!("{} -> {}[label=\"{}\", style=dotted];", item.0, item.1, item.2)
            }
        }
    }
}
