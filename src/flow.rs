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
        DoWhileStatement,
        ForStatement,
    },
    walker::{ Walker },
};

pub struct Flow<'a> {
    value: &'a json::JsonValue,
    source: &'a str, 
    edges: HashSet<(u32, u32)>,
    vertices: HashSet<String>,
    start: u32,
    stop: u32,
}

impl<'a> Flow<'a> {
    pub fn new(value: &'a json::JsonValue, source: &'a str) -> Self {
        Flow {
            value,
            source,
            edges: HashSet::new(),
            vertices: HashSet::new(),
            start: 0,
            stop: 1000000,
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
                    predecessors.dedup();
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
                                predecessors.dedup();
                                let mut t = self.traverse(tblocks, predecessors.clone());
                                let mut f = self.traverse(fblocks, predecessors.clone());
                                predecessors.clear();
                                predecessors.append(&mut t);
                                predecessors.append(&mut f);
                            }
                        },
                        GraphNode::DoWhileStatement(DoWhileStatement { condition, blocks }) => {
                            if let CodeBlock::Block(BlockContent { id, source }) = condition {
                                for _ in 0..2 {
                                    predecessors = self.traverse(blocks, predecessors.clone());
                                    let vertice = Flow::to_vertice(id, source, "diamond");
                                    self.vertices.insert(vertice);
                                    predecessors = predecessors
                                        .iter()
                                        .filter_map(|predecessor| {
                                            if !self.edges.insert((*predecessor, *id)) { return None; }
                                            Some(*id)
                                        })
                                    .collect::<Vec<u32>>();
                                    predecessors.dedup();
                                }
                                predecessors = vec![*id];
                            }
                        },
                        GraphNode::WhileStatement(WhileStatement { condition, blocks }) => {
                            if let CodeBlock::Block(BlockContent { id, source }) = condition {
                                let mut cond_predecessors = vec![];
                                for counter in 0..2 {
                                    let vertice = Flow::to_vertice(id, source, "diamond");
                                    self.vertices.insert(vertice);
                                    predecessors = predecessors
                                        .iter()
                                        .filter_map(|predecessor| {
                                            if !self.edges.insert((*predecessor, *id)) { return None; }
                                            Some(*id)
                                        })
                                    .collect::<Vec<u32>>();
                                    predecessors.dedup();
                                    if counter == 0 { cond_predecessors = predecessors.clone(); }
                                    predecessors = self.traverse(blocks, predecessors.clone());
                                }
                                predecessors = cond_predecessors;
                            }
                        },
                        GraphNode::ForStatement(ForStatement { init, condition, expression, blocks }) => {
                            let mut cond_predecessors = vec![];
                            if let CodeBlock::Block(BlockContent { id, source }) = init {
                                let vertice = Flow::to_vertice(id, source, "box");
                                self.vertices.insert(vertice);
                                predecessors = predecessors
                                    .iter()
                                    .filter_map(|predecessor| {
                                        if !self.edges.insert((*predecessor, *id)) { return None; }
                                        Some(*id)
                                    })
                                .collect::<Vec<u32>>();
                                predecessors.dedup();
                            }
                            for counter in 0..2 {
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
                                    predecessors.dedup();
                                    if counter == 0 { cond_predecessors = predecessors.clone(); }
                                }
                                predecessors = self.traverse(blocks, predecessors.clone());
                                if let CodeBlock::Block(BlockContent { id, source }) = expression {
                                    let vertice = Flow::to_vertice(id, source, "box");
                                    self.vertices.insert(vertice);
                                    predecessors = predecessors
                                        .iter()
                                        .filter_map(|predecessor| {
                                            if !self.edges.insert((*predecessor, *id)) { return None; }
                                            Some(*id)
                                        })
                                    .collect::<Vec<u32>>();
                                    predecessors.dedup();
                                }
                            }
                            predecessors = cond_predecessors;
                        },
                        GraphNode::Return(CodeBlock::Block(BlockContent { id, source })) 
                            | GraphNode::Revert(CodeBlock::Block(BlockContent { id, source }))
                            | GraphNode::Throw(CodeBlock::Block(BlockContent { id, source })) => {
                            let vertice = Flow::to_vertice(id, source, "box");
                            self.vertices.insert(vertice);
                            for predecessor in predecessors.iter() {
                                self.edges.insert((*predecessor, *id));
                            }
                            self.edges.insert((*id, self.stop));
                            return vec![];
                        },
                        GraphNode::Require(CodeBlock::Block(BlockContent { id, source }))
                            | GraphNode::Assert(CodeBlock::Block(BlockContent { id, source })) => {
                            let vertice = Flow::to_vertice(id, source, "diamond");
                            self.vertices.insert(vertice);
                            for predecessor in predecessors.iter() {
                                self.edges.insert((*predecessor, *id));
                            }
                            self.edges.insert((*id, self.stop));
                            predecessors = vec![*id];
                        },
                        GraphNode::Break(CodeBlock::Block(BlockContent { id, source })) => {
                        },
                        GraphNode::Continue(CodeBlock::Block(BlockContent { id, source })) => {
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
            self.vertices.insert(Flow::to_vertice(&self.start, "START", "circle"));
            self.vertices.insert(Flow::to_vertice(&self.stop, "STOP", "circle"));
            let predecessors = vec![self.start];
            self.traverse(blocks, predecessors);
        }
        println!("{}", self.to_dot());
    }
}
