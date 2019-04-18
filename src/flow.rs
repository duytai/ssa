use json;
use super::{
    graph::{ Graph, GraphNode, CodeBlock },
    walker::{ Walker },
};

pub struct Flow<'a> {
    value: &'a json::JsonValue,
    source: &'a str, 
}

pub struct SubDot {
    links: Vec<(u32, u32)>,
    nodes: Vec<String>,
}

impl<'a> Flow<'a> {
    pub fn new(value: &'a json::JsonValue, source: &'a str) -> Self {
        Flow { value, source }
    }

    pub fn render_dot(&self, prev_id: u32, cur_id: u32, next_id: u32, block: &CodeBlock) -> SubDot {
        let mut links = vec![];
        let mut nodes = vec![];
        match block {
            CodeBlock::Block(content) => {
                links.push((prev_id, cur_id));
                nodes.push(format!("  {}[label={:?}, shape=\"box\"];", cur_id, content));
            },
            CodeBlock::Link(_) => {
            },
            CodeBlock::None => {
            },
        }
        SubDot { links, nodes }
    }

    pub fn render(&self) {
        let walker = Walker::new(self.value);
        let mut graph = Graph::new(&walker, self.source);
        let root = graph.update();
        let mut links = vec![];
        let mut nodes = vec![];
        if let GraphNode::Root(blocks) = root {
            for (index, block) in blocks.iter().enumerate() {
                let mut subdot = self.render_dot(index as u32, index as u32 + 1, index as u32 + 2, block);
                links.append(&mut subdot.links);
                nodes.append(&mut subdot.nodes);
            }
        }
        let nodes = nodes.join("\n");
        let links = links
            .iter()
            .map(|link| format!("  {} -> {};", link.0, link.1))
            .collect::<Vec<String>>()
            .join("\n");
        let dot = format!("digraph {{\n{:0}\n{:1}\n}}", nodes, links);
        println!("{}", dot);
    }
}
