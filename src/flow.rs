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

pub struct SubDot {
    links: Vec<(String, String)>,
    nodes: Vec<String>,
}

impl<'a> Flow<'a> {
    pub fn new(value: &'a json::JsonValue, source: &'a str) -> Self {
        Flow { value, source }
    }

    pub fn render_dot(&self, prev_id: &str, cur_id: &str, next_id: &str, block: &CodeBlock) -> SubDot {
        let mut links = vec![];
        let mut nodes = vec![];
        match block {
            CodeBlock::Block(content) => {
                links.push((prev_id.to_string(), cur_id.to_string()));
                links.push((cur_id.to_string(), next_id.to_string()));
                nodes.push(format!("  {}[label={:?}, shape=\"box\"];", cur_id, content));
            },
            CodeBlock::Link(link) => {
                match &**link {
                    GraphNode::IfStatement(IfStatement { condition, tblocks, fblocks }) => {
                        if let CodeBlock::Block(content) = condition {
                            links.push((prev_id.to_string(), format!("A{}", cur_id)));
                            nodes.push(format!("  A{}[label={:?}, shape=\"box\"];", cur_id, content));
                        }
                    },
                    _ => {},
                }
            },
            CodeBlock::None => {},
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
                let index = index as u32;
                let prev_id = index.to_string();
                let cur_id = (index + 1).to_string();
                let next_id = (index + 2).to_string();
                let mut subdot = self.render_dot(&prev_id, &cur_id, &next_id, block);
                links.append(&mut subdot.links);
                nodes.append(&mut subdot.nodes);
                println!("{:?}", block);
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
