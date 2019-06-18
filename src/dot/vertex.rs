use crate::core::vertex::Shape;

pub enum DotVertex {
    Basic((u32, String, String))
}

impl From<(u32, &String, &Shape)> for DotVertex {
    fn from(item: (u32, &String, &Shape)) -> Self {
        let shape = match item.2 {
            Shape::Point => "point",
            Shape::Box => "box",
            Shape::Diamond => "diamond",
            Shape::DoubleCircle => "doublecircle",
            Shape::Mdiamond => "Mdiamond",
        };
        DotVertex::Basic((item.0, item.1.clone(), shape.to_string()))
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