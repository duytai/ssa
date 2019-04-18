use json;
use super::{
    graph::{
        Graph,
        GraphNode,
        CodeBlock,
        IfStatement,
    },
    walker::{ Walker },
};

pub struct Flow<'a> {
    value: &'a json::JsonValue,
    source: &'a str, 
}

impl<'a> Flow<'a> {
    pub fn new(value: &'a json::JsonValue, source: &'a str) -> Self {
        Flow { value, source }
    }

    pub fn render(&self) {
        let walker = Walker::new(self.value);
        let mut graph = Graph::new(&walker, self.source);
        let root = graph.update();
        println!("{:?}", root);
    }
}
