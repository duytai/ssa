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
        JumpKind,
    },
    dict::{ Dictionary },
    walker::{ Node },
    vertex::{ Vertex, Shape },
    analyzer::{ Analyzer, State },
};

pub struct ControlFlowGraph<'a> {
    jump_backs: HashMap<u32, u32>,
    entries: HashMap<u32, (u32, u32)>,
    edges: HashSet<(u32, u32)>,
    vertices: HashSet<Vertex>,
    dict: &'a Dictionary<'a>,
    counter: u32,
}

#[derive(Debug, Clone)]
struct CallParams {
    entry_id: u32,
    jump_back: Option<u32>,
    call_stack: Vec<u32>,
    kind: JumpKind,
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
            jump_backs: HashMap::new(),
            entries: HashMap::new(), 
            edges: HashSet::new(),
            vertices: HashSet::new(),
            dict,
            counter: 100000,
        }
    }

    pub fn traverse(&mut self, call_param: CallParams, blocks: &Vec<CodeBlock>, mut predecessors: Vec<u32>, breakers: &mut Vec<LoopBreaker>) -> Vec<u32> {
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
                                let mut t = self.traverse(call_param.clone(), tblocks, predecessors.clone(), breakers);
                                let mut f = self.traverse(call_param.clone(), fblocks, predecessors.clone(), breakers);
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
                                    predecessors = self.traverse(call_param.clone(), blocks, predecessors.clone(), &mut our_breakers);
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
                                    predecessors = self.traverse(call_param.clone(), blocks, predecessors.clone(), &mut our_breakers);
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
                                predecessors = self.traverse(call_param.clone(), blocks, predecessors.clone(), &mut our_breakers);
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
                            let stop = self.entries.get(&call_param.entry_id).unwrap().1;
                            self.edges.insert((id, stop));
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
                            let stop = self.entries.get(&call_param.entry_id).unwrap().1;
                            self.edges.insert((id, stop));
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
                        GraphNode::Jump(kind, from, to) => {
                            let (entry_start, _) = self.get_entry(*to);
                            let jump_back = self.get_jump_back(*from);
                            predecessors = predecessors
                                .iter()
                                .filter_map(|predecessor| {
                                    if !self.edges.insert((*predecessor, entry_start)) { return None; }
                                    Some(entry_start)
                                })
                                .collect::<Vec<u32>>();
                            if !predecessors.is_empty() {
                                let mut call_param = call_param.clone();
                                call_param.call_stack.push(call_param.entry_id);
                                call_param.entry_id = *to;
                                call_param.jump_back = Some(jump_back); 
                                call_param.kind = kind.clone();
                                self.jump(call_param);
                            }
                            predecessors = vec![jump_back];
                        },
                        GraphNode::PlaceHolder(CodeBlock::Block(_)) => {
                            let CallParams { ref call_stack, jump_back, .. } = call_param;
                            let jump_back = jump_back.unwrap();
                            for predecessor in predecessors.iter() {
                                self.edges.insert((*predecessor, jump_back));
                            }
                            let parent_entry_id = call_stack[call_stack.len() - 1];
                            let (_, stop) = self.get_entry(parent_entry_id);
                            predecessors = vec![stop];
                        },
                        _ => unimplemented!(),
                    }
                },
                CodeBlock::None => unimplemented!(), 
            }
        }
        return predecessors;
    }

    fn jump(&mut self, call_param: CallParams) {
        let CallParams { entry_id, ref kind, jump_back, .. } = call_param;
        let walker = self.dict.lookup(entry_id).expect("must exist").clone();
        let mut graph = Graph::new(walker);
        let root = graph.update();
        let (entry_start, entry_stop) = self.get_entry(entry_id);
        if let GraphNode::Root(blocks) = root {
            let mut predecessors = vec![entry_start];
            predecessors = self.traverse(call_param.clone(), blocks, predecessors, &mut vec![]);
            for predecessor in predecessors.iter() {
                self.edges.insert((*predecessor, entry_stop));
            }
        }
        match kind {
            JumpKind::Function => {
                let jump_back = jump_back.unwrap();
                self.edges.insert((entry_stop, jump_back));
            },
            JumpKind::Modifier => {
                let (_, stop) = self.get_entry(0);
                self.edges.insert((entry_stop, stop));
            },
        }
    }

    fn get_jump_back(&mut self, entry_id: u32) -> u32 {
        if !self.jump_backs.contains_key(&entry_id) {
            self.jump_backs.insert(entry_id, self.counter);
            self.vertices.insert(Vertex::new(self.counter, "", Shape::Point));
            self.counter += 1;
        }
        self.jump_backs.get(&entry_id).unwrap().clone()
    }

    fn get_entry(&mut self, entry_id: u32) -> (u32, u32) {
        if !self.entries.contains_key(&entry_id) {
            let tuple = (self.counter, self.counter + 1);
            self.counter += 2;
            self.entries.insert(entry_id, tuple);
            self.vertices.insert(Vertex::new(tuple.0, "", Shape::Point));
            self.vertices.insert(Vertex::new(tuple.1, "", Shape::Point));
        }
        self.entries.get(&entry_id).unwrap().clone()
    }

    pub fn analyze(&mut self, entry_id: u32, mut handlers: Vec<Box<Analyzer>>) {
        let walker = self.dict.lookup(entry_id).expect("must exist").clone();
        let mut graph = Graph::new(walker);
        let root = graph.update();
        let (start, stop) = self.get_entry(0);
        let (entry_start, entry_stop) = self.get_entry(entry_id);
        let call_param = CallParams {
            entry_id,
            jump_back: None,
            call_stack: vec![],
            kind: JumpKind::Function,
        };
        if let GraphNode::Root(blocks) = root {
            let mut predecessors = vec![entry_start];
            self.edges.insert((start, entry_start));
            predecessors = self.traverse(call_param, blocks, predecessors, &mut vec![]);
            for predecessor in predecessors.iter() {
                self.edges.insert((*predecessor, entry_stop));
            }
        }
        // Search for stop of this function
        let mut linked = false;
        for edge in self.edges.iter() {
            if edge.0 == entry_stop {
                linked = true;
                break;
            }
        }
        if !linked {
            self.edges.insert((entry_stop, stop));
        }
        // Analyze
        let mut state = State {
            edges: &self.edges,
            vertices: &self.vertices,
            dict: &self.dict,
            links: None,
            stop,
        };
        for handler in handlers.iter_mut() {
            handler.analyze(&mut state);
        }
    }
}
