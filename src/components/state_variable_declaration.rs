use crate::walker::{ Node };

pub struct StateVariableDeclaration {
    offsets: Vec<(u32, u32)>,
} 

impl StateVariableDeclaration {
    pub fn new() -> Self {
        StateVariableDeclaration { offsets: vec![] }
    }

    pub fn visit(&mut self, node: &Node, value: &json::JsonValue) {
        if node.level == 1 && node.name != "FunctionDefinition" {
            self.offsets.push((node.source_offset, node.source_len));
        }
    }

    pub fn report(&self, source: &str) -> Vec<String> {
        let mut parts: Vec<String> = vec![];
        for offset in &self.offsets {
            let from = offset.0 as usize;
            let to = (offset.0 + offset.1) as usize;
            let elem = &source[from..=to];
            parts.push(elem.to_string());
        }
        parts
    }
}
