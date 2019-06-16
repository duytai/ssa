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

    pub fn to_tuple(&self) -> (u32, String, String) {
        let shape = match self.shape {
            Shape::Point => "point",
            Shape::Box => "box",
            Shape::Diamond => "diamond",
            Shape::DoubleCircle => "doublecircle",
            Shape::Mdiamond => "Mdiamond",
        }.to_string();
        (self.id, self.source.clone(), shape)
    }
}
