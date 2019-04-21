use std::{
    collections::HashSet,
};
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

pub use super::graph::{ GraphKind };

pub struct Flow<'a> {
    value: &'a json::JsonValue,
    source: &'a str, 
    edges: HashSet<(u32, u32)>,
    vertices: HashSet<String>,
    start: u32,
    stop: u32,
}

#[derive(Debug, PartialEq)]
pub enum BreakerType {
    Continue,
    Break,
}

#[derive(Debug)]
pub struct LoopBreaker {
    kind: BreakerType,
    id: u32,
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

    pub fn traverse(&mut self, blocks: &Vec<CodeBlock>, predecessors: Vec<u32>, breakers: &mut Vec<LoopBreaker>) -> Vec<u32> {
        let mut predecessors = predecessors;
        for block in blocks {
            if predecessors.is_empty() { return vec![]; }
            match block {
                CodeBlock::Block(BlockContent { id, source }) => {
                    predecessors = predecessors
                        .iter()
                        .filter_map(|predecessor| {
                            if !self.edges.insert((*predecessor, *id)) { return None; }
                            Some(*id)
                        })
                        .collect::<Vec<u32>>();
                    if !predecessors.is_empty() {
                        let vertice = Flow::to_vertice(id, source, "box");
                        self.vertices.insert(vertice);
                    }
                    predecessors.dedup();
                },
                CodeBlock::Link(link) => {
                    match &**link {
                        GraphNode::IfStatement(IfStatement { condition, tblocks, fblocks }) => {
                            if let CodeBlock::Block(BlockContent { id, source }) = condition {
                                predecessors = predecessors
                                    .iter()
                                    .filter_map(|predecessor| {
                                        if !self.edges.insert((*predecessor, *id)) { return None; }
                                        Some(*id)
                                    })
                                .collect::<Vec<u32>>();
                                predecessors.dedup();
                                if !predecessors.is_empty() {
                                    let vertice = Flow::to_vertice(id, source, "diamond");
                                    self.vertices.insert(vertice);
                                }
                                let mut t = self.traverse(tblocks, predecessors.clone(), breakers);
                                let mut f = self.traverse(fblocks, predecessors.clone(), breakers);
                                predecessors.clear();
                                predecessors.append(&mut t);
                                predecessors.append(&mut f);
                            }
                        },
                        GraphNode::DoWhileStatement(DoWhileStatement { condition, blocks }) => {
                            if let CodeBlock::Block(BlockContent { id, source }) = condition {
                                let mut cond_predecessors = vec![];
                                let mut our_breakers = vec![];
                                for counter in 0..2 {
                                    predecessors = self.traverse(blocks, predecessors.clone(), &mut our_breakers);
                                    our_breakers
                                        .iter()
                                        .filter(|breaker| breaker.kind == BreakerType::Continue)
                                        .for_each(|LoopBreaker { id, .. }| {
                                            predecessors.push(*id);
                                        });
                                    predecessors = predecessors
                                        .iter()
                                        .filter_map(|predecessor| {
                                            if !self.edges.insert((*predecessor, *id)) { return None; }
                                            Some(*id)
                                        })
                                    .collect::<Vec<u32>>();
                                    predecessors.dedup();
                                    if !predecessors.is_empty() {
                                        let vertice = Flow::to_vertice(id, source, "diamond");
                                        self.vertices.insert(vertice);
                                    }
                                    if counter == 0 { cond_predecessors = predecessors.clone(); }
                                }
                                predecessors = cond_predecessors;
                                our_breakers
                                    .iter()
                                    .filter(|breaker| breaker.kind == BreakerType::Break)
                                    .for_each(|LoopBreaker { id, ..}| {
                                        predecessors.push(*id);
                                    });
                            }
                        },
                        GraphNode::WhileStatement(WhileStatement { condition, blocks }) => {
                            if let CodeBlock::Block(BlockContent { id, source }) = condition {
                                let mut cond_predecessors = vec![];
                                let mut our_breakers = vec![];
                                for counter in 0..2 {
                                    predecessors = predecessors
                                        .iter()
                                        .filter_map(|predecessor| {
                                            if !self.edges.insert((*predecessor, *id)) { return None; }
                                            Some(*id)
                                        })
                                    .collect::<Vec<u32>>();
                                    predecessors.dedup();
                                    if !predecessors.is_empty() {
                                        let vertice = Flow::to_vertice(id, source, "diamond");
                                        self.vertices.insert(vertice);
                                    }
                                    if counter == 0 { cond_predecessors = predecessors.clone(); }
                                    predecessors = self.traverse(blocks, predecessors.clone(), &mut our_breakers);
                                    our_breakers
                                        .iter()
                                        .filter(|breaker| breaker.kind == BreakerType::Continue)
                                        .for_each(|LoopBreaker { id, .. }| {
                                            predecessors.push(*id);
                                        });
                                }
                                predecessors = cond_predecessors;
                                our_breakers
                                    .iter()
                                    .filter(|breaker| breaker.kind == BreakerType::Break)
                                    .for_each(|LoopBreaker { id, ..}| {
                                        predecessors.push(*id);
                                    });
                            }
                        },
                        GraphNode::ForStatement(ForStatement { init, condition, expression, blocks }) => {
                            let mut cond_predecessors = vec![];
                            let mut our_breakers =  vec![];
                            if let CodeBlock::Block(BlockContent { id, source }) = init {
                                predecessors = predecessors
                                    .iter()
                                    .filter_map(|predecessor| {
                                        if !self.edges.insert((*predecessor, *id)) { return None; }
                                        Some(*id)
                                    })
                                .collect::<Vec<u32>>();
                                predecessors.dedup();
                                if !predecessors.is_empty() {
                                    let vertice = Flow::to_vertice(id, source, "box");
                                    self.vertices.insert(vertice);
                                }
                            }
                            for counter in 0..2 {
                                if let CodeBlock::Block(BlockContent { id, source }) = condition {
                                    predecessors = predecessors
                                        .iter()
                                        .filter_map(|predecessor| {
                                            if !self.edges.insert((*predecessor, *id)) { return None; }
                                            Some(*id)
                                        })
                                    .collect::<Vec<u32>>();
                                    predecessors.dedup();
                                    if !predecessors.is_empty() {
                                        let vertice = Flow::to_vertice(id, source, "diamond");
                                        self.vertices.insert(vertice);
                                    }
                                    if counter == 0 { cond_predecessors = predecessors.clone(); }
                                }
                                predecessors = self.traverse(blocks, predecessors.clone(), &mut our_breakers);
                                our_breakers
                                    .iter()
                                    .filter(|breaker| breaker.kind == BreakerType::Continue)
                                    .for_each(|LoopBreaker { id, .. }| {
                                        predecessors.push(*id);
                                    });
                                if let CodeBlock::Block(BlockContent { id, source }) = expression {
                                    predecessors = predecessors
                                        .iter()
                                        .filter_map(|predecessor| {
                                            if !self.edges.insert((*predecessor, *id)) { return None; }
                                            Some(*id)
                                        })
                                    .collect::<Vec<u32>>();
                                    predecessors.dedup();
                                    if !predecessors.is_empty() {
                                        let vertice = Flow::to_vertice(id, source, "box");
                                        self.vertices.insert(vertice);
                                    }
                                }
                            }
                            predecessors = cond_predecessors;
                            our_breakers
                                .iter()
                                .filter(|breaker| breaker.kind == BreakerType::Break)
                                .for_each(|LoopBreaker { id, ..}| {
                                    predecessors.push(*id);
                                });
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
                            predecessors = vec![];
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
                            let vertice = Flow::to_vertice(id, source, "box");
                            self.vertices.insert(vertice);
                            for predecessor in predecessors.iter() {
                                self.edges.insert((*predecessor, *id));
                            }
                            breakers.push(LoopBreaker { kind: BreakerType::Break, id: *id });
                            predecessors = vec![];
                        },
                        GraphNode::Continue(CodeBlock::Block(BlockContent { id, source })) => {
                            let vertice = Flow::to_vertice(id, source, "box");
                            self.vertices.insert(vertice);
                            for predecessor in predecessors.iter() {
                                self.edges.insert((*predecessor, *id));
                            }
                            breakers.push(LoopBreaker { kind: BreakerType::Continue, id: *id });
                            predecessors = vec![];
                        },
                        _ => {},
                    }
                },
                _ => {}, 
            }
        }
        return predecessors;
    }

    pub fn render(&mut self, kind: GraphKind) -> String {
        let walker = Walker::new(self.value);
        let mut graph = Graph::new(kind, &walker, self.source);
        let root = graph.update();
        if let GraphNode::Root(blocks) = root {
            self.vertices.insert(Flow::to_vertice(&self.start, "START", "circle"));
            self.vertices.insert(Flow::to_vertice(&self.stop, "STOP", "circle"));
            let mut predecessors = vec![self.start];
            predecessors = self.traverse(blocks, predecessors, &mut vec![]);
            for predecessor in predecessors {
                self.edges.insert((predecessor, self.stop));
            }
        }
        self.to_dot()
    }
}
