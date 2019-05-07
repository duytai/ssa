#[derive(PartialEq, Eq, Hash)]
pub enum Shape {
    Point,
    Box,
    Diamond,
    DoubleCircle,
}

#[derive(PartialEq, Eq, Hash)]
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

    pub fn to_string(&self) -> String {
        let shape = match self.shape {
            Shape::Point => "point",
            Shape::Box => "box",
            Shape::Diamond => "diamond",
            Shape::DoubleCircle => "doublecircle",
        };
        format!("  {}[label={:?}, shape=\"{}\"];\n", self.id, self.source, shape)
    }
}
