pub enum DotEdge {
    Basic((u32, u32)),
    Labeled((u32, u32, String)),
}

impl From<(u32, u32, &str)> for DotEdge {
    fn from(item: (u32, u32, &str)) -> Self {
        DotEdge::Labeled((item.0, item.1, item.2.to_string()))
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
