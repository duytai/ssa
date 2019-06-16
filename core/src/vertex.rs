#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Shape {
    Point,
    Box,
    Diamond,
    DoubleCircle,
    Mdiamond,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Vertex {
    pub id: u32,
    pub source: String,
    pub shape: Shape,
}

impl Vertex {
    pub fn new(id: u32, source: &str, shape: Shape) -> Self {
        Vertex {
            id,
            shape,
            source: source.to_string(),
        }
    }
}
