/// Shape represents function of a node in CFG 
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Shape {
    Entry,
    Statement,
    FunctionCall,
    IndexAccess,
    ConditionAndFunctionCall,
    ConditionAndIndexAccess,
    RootCondition,
}

/// Vertex in CFG
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Vertex {
    id: u32,
    source: String,
    shape: Shape,
    level: u32,
}

impl Vertex {
    pub fn new(id: u32, source: &str, shape: Shape, level: u32) -> Self {
        Vertex {
            id,
            shape,
            source: source.to_string(),
            level,
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

    pub fn get_level(&self) -> u32 {
        self.level
    }

    pub fn is_root_condition(&self) -> bool {
        self.shape == Shape::RootCondition
    }

    pub fn is_function_call(&self) -> bool {
        vec![
            Shape::FunctionCall,
            Shape::ConditionAndFunctionCall,
        ].contains(&self.shape)
    }
}
