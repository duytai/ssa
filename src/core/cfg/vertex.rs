/// Shape represents function of a node in CFG 
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Shape {
    Entry,
    Statement,
    FunctionCall,
    IndexAccess,
    ConditionAndFunctionCall,
    ConditionAndIndexAccess,
    Require,
    Assert,
    Throw,
    Suicide,
    Selfdestruct,
    Transfer,
    Revert,
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

    pub fn is_condition(&self) -> bool {
        vec![
            Shape::RootCondition,
            Shape::Require,
            Shape::Assert,
        ].contains(&self.shape)
    }

    pub fn is_stop(&self) -> bool {
        vec![
            Shape::Require,
            Shape::Assert,
            Shape::Revert,
            Shape::Suicide,
            Shape::Selfdestruct,
            Shape::Transfer,
            Shape::Throw,
        ].contains(&self.shape)
    }

    pub fn is_function_call(&self) -> bool {
        vec![
            Shape::FunctionCall,
            Shape::ConditionAndFunctionCall,
            Shape::Require,
            Shape::Assert,
            Shape::Revert,
            Shape::Suicide,
            Shape::Selfdestruct,
            Shape::Transfer,
        ].contains(&self.shape)
    }
}
