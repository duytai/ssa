use std::collections::HashSet;
use crate::graph::Graph;
use crate::block::{
    BlockNode,
    SimpleBlockNode,
    CodeBlock,
    IfStatement,
    WhileStatement,
    DoWhileStatement,
    ForStatement,
};
use crate::core:: {
    Dictionary,
    Node,
    Vertex,
    Shape,
    State,
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

    pub fn condition_traverse(&mut self, blocks: &Vec<SimpleBlockNode>) -> Vec<u32> {
        let mut chains = vec![];
        for (index, block) in blocks.iter().enumerate() {
            match block {
                SimpleBlockNode::FunctionCall(walker) => {
                    let Node { id, source, .. } = walker.node;
                    if index == blocks.len() - 1 {
                        let vertice = Vertex::new(id, source, Shape::Mdiamond);
                        self.vertices.insert(vertice);
                    } else {
                        let vertice = Vertex::new(id, source, Shape::DoubleCircle);
                        self.vertices.insert(vertice);
                    }
                    chains.push(id);
                },
                SimpleBlockNode::Unit(walker) => {
                    let Node { id, source, .. } = walker.node;
                    let vertice = Vertex::new(id, source, Shape::Diamond);
                    self.vertices.insert(vertice);
                    chains.push(id);
                },
                _ => unimplemented!(),
            }
        }
        for index in 0..chains.len() - 1 {
            self.edges.insert((chains[index], chains[index + 1]));
        }
        chains
    }

    pub fn simple_traverse(&mut self, blocks: &Vec<SimpleBlockNode>, mut predecessors: Vec<u32>, breakers: &mut Vec<LoopBreaker>) -> Vec<u32> {
        for block in blocks.iter() {
            if predecessors.is_empty() { return vec![]; }
            match block {
                SimpleBlockNode::Break(walker) => {
                    let Node { id, source, .. } = walker.node;
                    let vertice = Vertex::new(id, source, Shape::Box);
                    self.vertices.insert(vertice);
                    for predecessor in predecessors.iter() {
                        self.edges.insert((*predecessor, id));
                    }
                    breakers.push(LoopBreaker { kind: BreakerType::Break, id });
                    predecessors = vec![];
                },
                SimpleBlockNode::Continue(walker) => {
                    let Node { id, source, .. } = walker.node;
                    let vertice = Vertex::new(id, source, Shape::Box);
                    self.vertices.insert(vertice);
                    for predecessor in predecessors.iter() {
                        self.edges.insert((*predecessor, id));
                    }
                    breakers.push(LoopBreaker { kind: BreakerType::Continue, id });
                    predecessors = vec![];
                },
                SimpleBlockNode::Require(walker) | SimpleBlockNode::Assert(walker) => {
                    let Node { id, source, .. } = walker.node;
                    let vertice = Vertex::new(id, source, Shape::DoubleCircle);
                    self.vertices.insert(vertice);
                    for predecessor in predecessors.iter() {
                        self.edges.insert((*predecessor, id));
                    }
                    self.edges.insert((id, self.stop));
                    predecessors = vec![id];
                },
                SimpleBlockNode::Throw(walker) => {
                    let Node { id, source, .. } = walker.node;
                    let vertice = Vertex::new(id, source, Shape::Box);
                    self.vertices.insert(vertice);
                    for predecessor in predecessors.iter() {
                        self.edges.insert((*predecessor, id));
                    }
                    self.edges.insert((id, self.stop));
                    predecessors = vec![];
                },
                SimpleBlockNode::Revert(walker) 
                    | SimpleBlockNode::Selfdestruct(walker)
                    | SimpleBlockNode::Suicide(walker) => {
                    let Node { id, source, .. } = walker.node;
                    let vertice = Vertex::new(id, source, Shape::DoubleCircle);
                    self.vertices.insert(vertice);
                    for predecessor in predecessors.iter() {
                        self.edges.insert((*predecessor, id));
                    }
                    self.edges.insert((id, self.stop));
                    predecessors = vec![];
                },
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
                SimpleBlockNode::None => unimplemented!(),
            }
        }
        return predecessors;
    }

    pub fn traverse(&mut self, blocks: &Vec<CodeBlock>, mut predecessors: Vec<u32>, breakers: &mut Vec<LoopBreaker>) -> Vec<u32> {
        for block in blocks {
            if predecessors.is_empty() { return vec![]; }
            match block {
                CodeBlock::Block(walker) => {
                    let simple_blocks = Graph::split(walker.clone());
                    predecessors = self.simple_traverse(&simple_blocks, predecessors.clone(), breakers);
                },
                CodeBlock::Link(link) => {
                    match &**link {
                        BlockNode::IfStatement(IfStatement { condition, tblocks, fblocks }) => {
                            if let CodeBlock::Block(walker) = condition {
                                let condition_blocks = Graph::split(walker.clone());
                                let chains = self.condition_traverse(&condition_blocks);
                                if !chains.is_empty() {
                                    for predecessor in predecessors.iter() {
                                        self.edges.insert((*predecessor, chains[0]));
                                    }
                                    predecessors = vec![chains[chains.len() - 1]];
                                    let mut t = self.traverse(tblocks, predecessors.clone(), breakers);
                                    let mut f = self.traverse(fblocks, predecessors.clone(), breakers);
                                    predecessors.clear();
                                    predecessors.append(&mut t);
                                    predecessors.append(&mut f);
                                }
                            }
                        },
                        BlockNode::DoWhileStatement(DoWhileStatement { condition, blocks }) => {
                            if let CodeBlock::Block(walker) = condition {
                                let mut our_breakers = vec![];
                                predecessors = self.traverse(blocks, predecessors.clone(), &mut our_breakers);
                                our_breakers
                                    .iter()
                                    .filter(|breaker| breaker.kind == BreakerType::Continue)
                                    .for_each(|LoopBreaker { id, .. }| {
                                        predecessors.push(*id);
                                    });
                                if !predecessors.is_empty() {
                                    let condition_blocks = Graph::split(walker.clone());
                                    let chains = self.condition_traverse(&condition_blocks);
                                    if !chains.is_empty() {
                                        for predecessor in predecessors.iter() {
                                            self.edges.insert((*predecessor, chains[0]));
                                        }
                                    }
                                    predecessors = vec![chains[chains.len() - 1]];
                                    self.traverse(blocks, predecessors.clone(), &mut our_breakers);
                                }
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
                                let mut our_breakers = vec![];
                                let condition_blocks = Graph::split(walker.clone());
                                let chains = self.condition_traverse(&condition_blocks);
                                if !chains.is_empty() {
                                    for predecessor in predecessors.iter() {
                                        self.edges.insert((*predecessor, chains[0]));
                                    }
                                    predecessors = vec![chains[chains.len() - 1]];
                                    predecessors = self.traverse(blocks, predecessors.clone(), &mut our_breakers);
                                    our_breakers
                                        .iter()
                                        .filter(|breaker| breaker.kind == BreakerType::Continue)
                                        .for_each(|LoopBreaker { id, .. }| {
                                            predecessors.push(*id);
                                        });
                                    for predecessor in predecessors.iter() {
                                        self.edges.insert((*predecessor, chains[0]));
                                    }
                                    predecessors = vec![chains[chains.len() - 1]];
                                    our_breakers
                                        .iter()
                                        .filter(|breaker| breaker.kind == BreakerType::Break)
                                        .for_each(|LoopBreaker { id, ..}| {
                                            predecessors.push(*id);
                                        });
                                }
                            }
                        },
                        BlockNode::ForStatement(ForStatement { init, condition, expression, blocks }) => {
                            let mut our_breakers = vec![];
                            let mut cond_predecessors = vec![];
                            if let CodeBlock::Block(walker) = init {
                                let simple_blocks = Graph::split(walker.clone());
                                predecessors = self.simple_traverse(&simple_blocks, predecessors.clone(), breakers);
                            }
                            for _ in 0..2 {
                                if let CodeBlock::Block(walker) = condition {
                                    let condition_blocks = Graph::split(walker.clone());
                                    let chains = self.condition_traverse(&condition_blocks);
                                    if !chains.is_empty() {
                                        for predecessor in predecessors.iter() {
                                            self.edges.insert((*predecessor, chains[0]));
                                        }
                                        predecessors = vec![chains[chains.len() - 1]];
                                        cond_predecessors = vec![chains[chains.len() - 1]];
                                    }
                                }
                                predecessors = self.traverse(blocks, predecessors.clone(), &mut our_breakers);
                                our_breakers
                                    .iter()
                                    .filter(|breaker| breaker.kind == BreakerType::Continue)
                                    .for_each(|LoopBreaker { id, .. }| {
                                        predecessors.push(*id);
                                    });
                                if let CodeBlock::Block(walker) = expression {
                                    let simple_blocks = Graph::split(walker.clone());
                                    predecessors = self.simple_traverse(&simple_blocks, predecessors.clone(), breakers);
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
                        BlockNode::Return(blocks) => {
                            predecessors = self.simple_traverse(blocks, predecessors.clone(), breakers);
                        },
                        BlockNode::Root(_) => unimplemented!(),
                        BlockNode::None => unimplemented!(),
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

    pub fn start_at(&mut self, entry_id: u32) -> Option<State> {
        let walker = self.dict.lookup(entry_id).expect("must exist").clone();
        let entry_names = vec!["FunctionDefinition", "ModifierDefinition"];
        if entry_names.contains(&walker.node.name) {
            let mut graph = Graph::new(walker);
            let root = graph.update();
            let states = self.dict.lookup_states(entry_id);
            if let BlockNode::Root(blocks) = root {
                for id in vec![self.start, self.stop] {
                    let vertex = Vertex::new(id, "", Shape::Point);
                    self.vertices.insert(vertex);
                }
                let last_id = states.iter().fold(self.start, |prev, cur| {
                    let vertex = Vertex::new(cur.node.id, cur.node.source, Shape::Box);
                    self.vertices.insert(vertex);
                    self.edges.insert((prev, cur.node.id));
                    cur.node.id
                });
                let predecessors = self.traverse(blocks, vec![last_id], &mut vec![]);
                for predecessor in predecessors.iter() {
                    self.edges.insert((*predecessor, self.stop));
                }
            }
            return Some(State {
                edges: &self.edges,
                vertices: &self.vertices,
                dict: &self.dict,
                stop: self.stop,
            });
        }
        return None;
    }
}