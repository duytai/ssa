use crate::walker::{ Contract };

pub struct StateVariableDeclaration {
    level: u32,
} 

impl StateVariableDeclaration {
    pub fn new() -> Self {
        StateVariableDeclaration { level: 0 }
    }

    pub fn visit(&mut self, contract: &Contract) {
        println!("contract: {}", contract.name);
    }

    // pub fn visit(&mut self, node: &Node, value: &json::JsonValue) {
        // if node.name == "ContractDefinition" { self.level = node.level + 1 }
    // }
}
