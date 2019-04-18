use json;
use super::{
    graph::{ Graph, GraphNode, CodeBlock },
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
        let mut links: Vec<(u32, u32)> = vec![];
        let mut dot = vec![String::from("digraph {")];
        let root = graph.update();
        if let GraphNode::Root(blocks) = root {
            let mut prev_id = 0;
            dot.push(format!("  {}[label=\"Entry\", shape=\"circle\"];", prev_id));
            for block in blocks {
                match block {
                    CodeBlock::Block(content) => {
                        dot.push(format!("  {}[label={:?}, shape=\"box\"];", prev_id + 1, content.trim()));
                        links.push((prev_id, prev_id + 1));
                        prev_id = prev_id + 1;
                    },
                    _ => {},
                }
            }
        }
        for link in links {
            dot.push(format!("  {} -> {};", link.0, link.1));
        } 
        dot.push(String::from("}"));
        println!("{}", dot.join("\n"));
    }
}
