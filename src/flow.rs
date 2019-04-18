use std::collections::HashSet;
use json;
use super::{
    graph::{
        Graph,
        GraphNode,
        CodeBlock,
        BlockContent,
    },
    walker::{ Walker },
};

pub struct Flow<'a> {
    value: &'a json::JsonValue,
    source: &'a str, 
    edges: HashSet<(u32, u32)>,
    vertices: HashSet<String>,
}

impl<'a> Flow<'a> {
    pub fn new(value: &'a json::JsonValue, source: &'a str) -> Self {
        Flow {
            value,
            source,
            edges: HashSet::new(),
            vertices: HashSet::new(),
        }
    }

    pub fn to_vertice(id: &u32, source: &str) -> String {
        format!("  {}[label={:?}, shape=\"box\"];\n", id, source)
    }

    pub fn to_edge(e: &(u32, u32)) -> String {
        format!("  {} -> {};\n", e.0, e.1)
    }

    pub fn to_dot(&self) -> String {
        let mut vertices = String::from("");
        let mut edges = String::from("");
        for edge in &self.edges {
            edges.push_str(&Flow::to_edge(edge));
        }
        for vertice in &self.vertices {
            vertices.push_str(vertice);
        }
        format!("digraph {{\n{0}{1}}}", vertices, edges)
    }

    pub fn traverse(&mut self, blocks: &Vec<CodeBlock>, vertex: u32) {
        let mut vertex = vertex;
        for block in blocks {
            match block {
                CodeBlock::Block(BlockContent { id, source }) => {
                    let vertice = Flow::to_vertice(id, source);
                    self.vertices.insert(vertice);
                    if !self.edges.insert((vertex, *id)) { return; }
                    vertex = *id;
                },
                _ => {},
            }
        }
    }

    pub fn render(&mut self) {
        let walker = Walker::new(self.value);
        let mut graph = Graph::new(&walker, self.source);
        let root = graph.update();
        if let GraphNode::Root(blocks) = root {
            let vertex = 0;
            let vertice = Flow::to_vertice(&0, "START");
            self.vertices.insert(vertice);
            self.traverse(blocks, vertex);
        }
        println!("{}", self.to_dot());
    }
}
