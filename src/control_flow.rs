use std::collections::HashSet;
use crate::{
    code_block::{
        BlockNode,
        SimpleBlockNode,
        CodeBlock,
        IfStatement,
        WhileStatement,
        DoWhileStatement,
        ForStatement,
    },
    graph::Graph,
    dict::Dictionary,
    walker::Node,
    vertex::{ Vertex, Shape },
    analyzer::{ Analyzer, State },
};

pub struct ControlFlowGraph<'a> {
    edges: HashSet<(u32, u32)>,
    vertices: HashSet<Vertex>,
    dict: &'a Dictionary<'a>,
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

impl<'a> ControlFlowGraph<'a> {
    pub fn new(dict: &'a Dictionary) -> Self {
        ControlFlowGraph {
            edges: HashSet::new(),
            vertices: HashSet::new(),
            dict,
            start: 0,
            stop: 1000000,
        }
    }

    pub fn simple_traverse(&mut self, blocks: &Vec<SimpleBlockNode>, mut predecessors: Vec<u32>, breakers: &mut Vec<LoopBreaker>) -> Vec<u32> {
        for block in blocks {
            if predecessors.is_empty() { return vec![]; }
            match block {
                SimpleBlockNode::Throw(walker) => {},
                SimpleBlockNode::Break(walker) => {},
                SimpleBlockNode::Continue(walker) => {},
                SimpleBlockNode::Require(walker) => {},
                SimpleBlockNode::Assert(walker) => {},
                SimpleBlockNode::Revert(walker) => {},
                SimpleBlockNode::Suicide(walker) => {},
                SimpleBlockNode::Selfdestruct(walker) => {},
                SimpleBlockNode::FunctionCall(walker) => {
                    let Node { id, source, .. } = walker.node;
                    predecessors = predecessors
                        .iter()
                        .filter_map(|predecessor| {
                            if !self.edges.insert((*predecessor, id)) { return None; }
                            Some(id)
                        })
                    .collect::<Vec<u32>>();
                    if !predecessors.is_empty() {
                        let vertice = Vertex::new(id, source, Shape::DoubleCircle);
                        self.vertices.insert(vertice);
                    }
                    predecessors.dedup();
                },
                SimpleBlockNode::Unit(walker) => {
                    let Node { id, source, .. } = walker.node;
                    predecessors = predecessors
                        .iter()
                        .filter_map(|predecessor| {
                            if !self.edges.insert((*predecessor, id)) { return None; }
                            Some(id)
                        })
                    .collect::<Vec<u32>>();
                    if !predecessors.is_empty() {
                        let vertice = Vertex::new(id, source, Shape::Box);
                        self.vertices.insert(vertice);
                    }
                    predecessors.dedup();
                },
                SimpleBlockNode::None => {},
            }
        }
        return predecessors;
    }

    pub fn traverse(&mut self, blocks: &Vec<CodeBlock>, mut predecessors: Vec<u32>, breakers: &mut Vec<LoopBreaker>) -> Vec<u32> {
        for block in blocks {
            if predecessors.is_empty() { return vec![]; }
            match block {
                CodeBlock::Block(walker) => {
                    let blocks = Graph::split(walker.clone());
                    predecessors = self.simple_traverse(&blocks, predecessors.clone(), breakers);
                },
                CodeBlock::Link(link) => {
                    match &**link {
                        BlockNode::IfStatement(IfStatement { condition, tblocks, fblocks }) => {
                            if let CodeBlock::Block(walker) = condition {
                                let Node { id, source, .. } = walker.node;
                                predecessors = predecessors
                                    .iter()
                                    .filter_map(|predecessor| {
                                        if !self.edges.insert((*predecessor, id)) { return None; }
                                        Some(id)
                                    })
                                .collect::<Vec<u32>>();
                                predecessors.dedup();
                                if !predecessors.is_empty() {
                                    let vertice = Vertex::new(id, source, Shape::Diamond);
                                    self.vertices.insert(vertice);
                                }
                                let mut t = self.traverse(tblocks, predecessors.clone(), breakers);
                                let mut f = self.traverse(fblocks, predecessors.clone(), breakers);
                                predecessors.clear();
                                predecessors.append(&mut t);
                                predecessors.append(&mut f);
                            }
                        },
                        BlockNode::DoWhileStatement(DoWhileStatement { condition, blocks }) => {
                            if let CodeBlock::Block(walker) = condition {
                                let mut cond_predecessors = vec![];
                                let mut our_breakers = vec![];
                                let Node { id, source, .. } = walker.node;
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
                                            if !self.edges.insert((*predecessor, id)) { return None; }
                                            Some(id)
                                        })
                                    .collect::<Vec<u32>>();
                                    predecessors.dedup();
                                    if !predecessors.is_empty() {
                                        let vertice = Vertex::new(id, source, Shape::Diamond);
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
                        BlockNode::WhileStatement(WhileStatement { condition, blocks }) => {
                            if let CodeBlock::Block(walker) = condition {
                                let mut cond_predecessors = vec![];
                                let mut our_breakers = vec![];
                                let Node { id, source, .. } = walker.node;
                                for counter in 0..2 {
                                    predecessors = predecessors
                                        .iter()
                                        .filter_map(|predecessor| {
                                            if !self.edges.insert((*predecessor, id)) { return None; }
                                            Some(id)
                                        })
                                    .collect::<Vec<u32>>();
                                    predecessors.dedup();
                                    if !predecessors.is_empty() {
                                        let vertice = Vertex::new(id, source, Shape::Diamond);
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
                        BlockNode::ForStatement(ForStatement { init, condition, expression, blocks }) => {
                            let mut cond_predecessors = vec![];
                            let mut our_breakers =  vec![];
                            if let CodeBlock::Block(walker) = init {
                                let Node { id, source, .. } = walker.node;
                                predecessors = predecessors
                                    .iter()
                                    .filter_map(|predecessor| {
                                        if !self.edges.insert((*predecessor, id)) { return None; }
                                        Some(id)
                                    })
                                .collect::<Vec<u32>>();
                                predecessors.dedup();
                                if !predecessors.is_empty() {
                                    let vertice = Vertex::new(id, source, Shape::Box);
                                    self.vertices.insert(vertice);
                                }
                            }
                            for counter in 0..2 {
                                if let CodeBlock::Block(walker) = condition {
                                    let Node { id, source, .. } = walker.node;
                                    predecessors = predecessors
                                        .iter()
                                        .filter_map(|predecessor| {
                                            if !self.edges.insert((*predecessor, id)) { return None; }
                                            Some(id)
                                        })
                                    .collect::<Vec<u32>>();
                                    predecessors.dedup();
                                    if !predecessors.is_empty() {
                                        let vertice = Vertex::new(id, source, Shape::Diamond) ;
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
                                if let CodeBlock::Block(walker) = expression {
                                    let Node { id, source, .. } = walker.node;
                                    predecessors = predecessors
                                        .iter()
                                        .filter_map(|predecessor| {
                                            if !self.edges.insert((*predecessor, id)) { return None; }
                                            Some(id)
                                        })
                                    .collect::<Vec<u32>>();
                                    predecessors.dedup();
                                    if !predecessors.is_empty() {
                                        let vertice = Vertex::new(id, source, Shape::Box);
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
                        BlockNode::Return(_) => {},
                        BlockNode::Root(_) => {},
                        BlockNode::None => {},
                    }
                },
                CodeBlock::SimpleBlocks(blocks) => {
                    predecessors = self.simple_traverse(blocks, predecessors.clone(), breakers);
                },
                CodeBlock::None => unimplemented!(), 
            }
        }
        return predecessors;
    }

    pub fn analyze(&mut self, entry_id: u32, mut handlers: Vec<Box<Analyzer>>) {
        let walker = self.dict.lookup(entry_id).expect("must exist").clone();
        let mut graph = Graph::new(walker);
        let root = graph.update();
        if let BlockNode::Root(blocks) = root {
            let vertex = Vertex::new(self.start, "START", Shape::Point);
            self.vertices.insert(vertex);
            let vertex = Vertex::new(self.stop, "STOP", Shape::Point);
            self.vertices.insert(vertex);
            let mut predecessors = vec![self.start];
            predecessors = self.traverse(blocks, predecessors, &mut vec![]);
            for predecessor in predecessors.iter() {
                self.edges.insert((*predecessor, self.stop));
            }
        }
        let mut state = State {
            edges: &self.edges,
            vertices: &self.vertices,
            dict: &self.dict,
            links: None,
            stop: self.stop,
        };
        for handler in handlers.iter_mut() {
            handler.analyze(&mut state);
        }
    }
}
