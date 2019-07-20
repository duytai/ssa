/// Shape represents function of a node in CFG 
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Shape {
    Point,
    Box,
    Diamond,
    DoubleCircle,
}

/// Vertex in CFG
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Vertex {
    id: u32,
    source: String,
    shape: Shape,
}

impl Vertex {
    pub fn new(id: u32, source: &str, shape: Shape) -> Self {
        Vertex {
            id,
            shape,
            source: source.to_string(),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_source(&self) -> &str {
        &self.source
    }

    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }
}
