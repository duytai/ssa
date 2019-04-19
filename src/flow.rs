use std::collections::HashSet;
use json;
use super::{
    graph::{
        Graph,
        GraphNode,
        CodeBlock,
        BlockContent,
        IfStatement,
        WhileStatement,
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

    pub fn to_vertice(id: &u32, source: &str, shape: &str) -> String {
        format!("  {}[label={:?}, shape=\"{}\"];\n", id, source, shape)
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

    pub fn traverse(&mut self, blocks: &Vec<CodeBlock>, predecessors: Vec<u32>) -> Vec<u32> {
        let mut predecessors = predecessors;
        for block in blocks {
            if predecessors.is_empty() { return vec![]; }
            match block {
                CodeBlock::Block(BlockContent { id, source }) => {
                    let vertice = Flow::to_vertice(id, source, "box");
                    self.vertices.insert(vertice);
                    predecessors = predecessors
                        .iter()
                        .filter_map(|predecessor| {
                            if !self.edges.insert((*predecessor, *id)) { return None; }
                            Some(*id)
                        })
                        .collect::<Vec<u32>>();
                },
                CodeBlock::Link(link) => {
                    match &**link {
                        GraphNode::IfStatement(IfStatement { condition, tblocks, fblocks }) => {
                            if let CodeBlock::Block(BlockContent { id, source }) = condition {
                                let vertice = Flow::to_vertice(id, source, "diamond");
                                self.vertices.insert(vertice);
                                predecessors = predecessors
                                    .iter()
                                    .filter_map(|predecessor| {
                                        if !self.edges.insert((*predecessor, *id)) { return None; }
                                        Some(*id)
                                    })
                                .collect::<Vec<u32>>();
                                let mut t = self.traverse(tblocks, predecessors.clone());
                                let mut f = self.traverse(fblocks, predecessors.clone());
                                predecessors.clear();
                                predecessors.append(&mut t);
                                predecessors.append(&mut f);
                            }
                        },
                        GraphNode::WhileStatement(WhileStatement { condition, blocks }) => {
                            if let CodeBlock::Block(BlockContent { id, source }) = condition {
                                let vertice = Flow::to_vertice(id, source, "diamond");
                                self.vertices.insert(vertice);
                                predecessors = predecessors
                                    .iter()
                                    .filter_map(|predecessor| {
                                        if !self.edges.insert((*predecessor, *id)) { return None; }
                                        Some(*id)
                                    })
                                .collect::<Vec<u32>>();
                                predecessors = self.traverse(blocks, predecessors.clone());
                                for predecessor in &predecessors {
                                    self.edges.insert((*predecessor, *id));
                                }
                            }
                        },
                        _ => {},
                    }
                },
                _ => {}, 
            }
        }
        return predecessors;
    }

    pub fn render(&mut self) {
        let walker = Walker::new(self.value);
        let mut graph = Graph::new(&walker, self.source);
        let root = graph.update();
        if let GraphNode::Root(blocks) = root {
            let predecessors = vec![0];
            let vertice = Flow::to_vertice(&predecessors[0], "START", "circle");
            self.vertices.insert(vertice);
            self.traverse(blocks, predecessors);
        }
        println!("{}", self.to_dot());
    }
}
