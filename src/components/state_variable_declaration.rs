use crate::walker::{ Node };

pub struct StateVariableDeclaration {
    level: u32,
} 

impl StateVariableDeclaration {
    pub fn new() -> Self {
        StateVariableDeclaration { level: 0 }
    }

    pub fn visit(&mut self, node: &Node, value: &json::JsonValue) {
        println!("name: {}", node.name);
    }
}
