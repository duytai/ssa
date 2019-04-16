use crate::walker::{ Node };

pub struct FunctionDefinition {
} 

impl FunctionDefinition {
    pub fn new() -> Self {
        FunctionDefinition {}
    }

    pub fn visit(&mut self, node: &Node, value: &json::JsonValue) {
        if node.level == 1 && node.name != "FunctionDefinition" {
        }
    }

    pub fn report(&self, source: &str) -> Vec<String> {
        vec![]
    }
}
