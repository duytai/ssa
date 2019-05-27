use std::{
    collections::{
        HashMap,
        HashSet,
    },
};
use super::{
    graph::{
        Graph,
        GraphNode,
        CodeBlock,
        IfStatement,
        WhileStatement,
        DoWhileStatement,
        ForStatement,
    },
    dict::{ Dictionary },
    walker::{ Node },
    vertex::{ Vertex, Shape },
    analyzer::{ Analyzer, State },
};

pub struct ControlFlowGraph<'a> {
    entries: HashMap<u32, Vec<u32>>,
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
            entries: HashMap::new(), 
            edges: HashSet::new(),
            vertices: HashSet::new(),
            dict,
            start: 0,
            stop: 1000000,
        }
    }

    pub fn traverse(&mut self, blocks: &Vec<CodeBlock>, mut predecessors: Vec<u32>, breakers: &mut Vec<LoopBreaker>, to: u32) -> Vec<u32> {
        for block in blocks {
            if predecessors.is_empty() {
                return vec![];
            }
            match block {
                CodeBlock::Block(walker) => {
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
                CodeBlock::Link(link) => {
                    match &**link {
                        GraphNode::IfStatement(IfStatement { condition, tblocks, fblocks }) => {
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
                                let mut t = self.traverse(tblocks, predecessors.clone(), breakers, to);
                                let mut f = self.traverse(fblocks, predecessors.clone(), breakers, to);
                                predecessors.clear();
                                predecessors.append(&mut t);
                                predecessors.append(&mut f);
                            }
                        },
                        GraphNode::DoWhileStatement(DoWhileStatement { condition, blocks }) => {
                            if let CodeBlock::Block(walker) = condition {
                                let mut cond_predecessors = vec![];
                                let mut our_breakers = vec![];
                                let Node { id, source, .. } = walker.node;
                                for counter in 0..2 {
                                    predecessors = self.traverse(blocks, predecessors.clone(), &mut our_breakers, to);
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
                        GraphNode::WhileStatement(WhileStatement { condition, blocks }) => {
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
                                    predecessors = self.traverse(blocks, predecessors.clone(), &mut our_breakers, to);
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
                                predecessors = self.traverse(blocks, predecessors.clone(), &mut our_breakers, to);
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
                        GraphNode::Return(CodeBlock::Block(walker)) 
                            | GraphNode::Revert(CodeBlock::Block(walker))
                            | GraphNode::Throw(CodeBlock::Block(walker)) 
                            | GraphNode::Suicide(CodeBlock::Block(walker)) 
                            | GraphNode::Selfdestruct(CodeBlock::Block(walker)) => {
                            let Node { id, source, .. } = walker.node;
                            let vertice = Vertex::new(id, source, Shape::Box);
                            self.vertices.insert(vertice);
                            for predecessor in predecessors.iter() {
                                self.edges.insert((*predecessor, id));
                            }
                            self.edges.insert((id, self.stop));
                            predecessors = vec![];
                        },
                        GraphNode::Require(CodeBlock::Block(walker))
                            | GraphNode::Assert(CodeBlock::Block(walker)) => {
                            let Node { id, source, .. } = walker.node;
                            let vertice = Vertex::new(id, source, Shape::Diamond);
                            self.vertices.insert(vertice);
                            for predecessor in predecessors.iter() {
                                self.edges.insert((*predecessor, id));
                            }
                            self.edges.insert((id, self.stop));
                            predecessors = vec![id];
                        },
                        GraphNode::Break(CodeBlock::Block(walker)) => {
                            let Node { id, source, .. } = walker.node;
                            let vertice = Vertex::new(id, source, Shape::Box);
                            self.vertices.insert(vertice);
                            for predecessor in predecessors.iter() {
                                self.edges.insert((*predecessor, id));
                            }
                            breakers.push(LoopBreaker { kind: BreakerType::Break, id });
                            predecessors = vec![];
                        },
                        GraphNode::Continue(CodeBlock::Block(walker)) => {
                            let Node { id, source, .. } = walker.node;
                            let vertice = Vertex::new(id, source, Shape::Box);
                            self.vertices.insert(vertice);
                            for predecessor in predecessors.iter() {
                                self.edges.insert((*predecessor, id));
                            }
                            breakers.push(LoopBreaker { kind: BreakerType::Continue, id });
                            predecessors = vec![];
                        },
                        GraphNode::FunctionCall(CodeBlock::Block(walker)) => {
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
                        GraphNode::ModifierInvocation(CodeBlock::Block(walker)) => {
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
                        GraphNode::Jump(to) => {
                            // TODO: jump to other ref
                            let p = self.jump(*to, predecessors, *to);
                            predecessors = vec![]
                        },
                        GraphNode::PlaceHolder(CodeBlock::Block(walker)) => {
                            // TODO: jump back to prev
                            panic!("PLACE HOLDER");
                        },
                        _ => unimplemented!(),
                    }
                },
                CodeBlock::None => unimplemented!(), 
            }
        }
        return predecessors;
    }

    fn jump(&mut self, entry: u32, mut predecessors: Vec<u32>, to: u32) -> Vec<u32> {
        if self.entries.contains_key(&entry) {
            for predecessor in predecessors {
                self.edges.insert((predecessor, entry));
            }
            self.entries.get(&entry).unwrap().clone()
        } else {
            let walker = self.dict.lookup(entry).expect("must exist").clone();
            let mut graph = Graph::new(walker);
            let root = graph.update();
            if let GraphNode::Root(blocks) = root {
                self.traverse(blocks, predecessors, &mut vec![], to)
            } else {
                vec![]
            }
        }
    }

    pub fn analyze(&mut self, entry: u32, mut handlers: Vec<Box<Analyzer>>) {
        let walker = self.dict.lookup(entry).expect("must exist").clone();
        let mut graph = Graph::new(walker);
        let root = graph.update();
        if let GraphNode::Root(blocks) = root {
            let vertice = Vertex::new(self.start, "START", Shape::Point);
            self.vertices.insert(vertice);
            let vertice = Vertex::new(self.stop, "STOP", Shape::Point);
            self.vertices.insert(vertice);
            let mut predecessors = vec![self.start];
            self.entries.insert(entry, vec![]);
            predecessors = self.traverse(blocks, predecessors, &mut vec![], self.stop);
            for predecessor in predecessors.iter() {
                self.edges.insert((*predecessor, self.stop));
            }
            self.entries.insert(entry, predecessors);
        }
        let mut state = State {
            edges: &self.edges,
            vertices: &self.vertices,
            dict: &self.dict,
            links: None,
        };
        for handler in handlers.iter_mut() {
            handler.analyze(&mut state);
        }
    }
}
