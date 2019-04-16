use crate::walker::{ Node };

pub struct StateVariableDeclaration {
} 

impl StateVariableDeclaration {
    pub fn new() -> Self {
        StateVariableDeclaration {}
    }

    pub fn visit(&self, node: &Node, value: &json::JsonValue) {
        if node.name == "ContractDefinition" {
        }
    }
}
