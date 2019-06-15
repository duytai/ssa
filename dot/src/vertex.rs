pub enum DotVertex {
    Basic((u32, String, String))
}

impl From<(u32, String, String)> for DotVertex {
    fn from(item: (u32, String, String)) -> Self {
        DotVertex::Basic(item)
    }
}

impl DotVertex {
    pub fn format(&self) -> String {
        match self {
            DotVertex::Basic((id, source, shape)) => {
                format!("{}[label={:?}, shape=\"{}\"];", id, source, shape)
            }
        }
    }
}
