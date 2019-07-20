use std::collections::HashSet;
use crate::cfg::{
    Graph,
    BlockNode,
    SimpleBlockNode,
    CodeBlock,
    IfStatement,
    WhileStatement,
    DoWhileStatement,
    ForStatement,
};
use crate::core::{
    LookupInputType,
    Dictionary,
    Node,
    Vertex,
    Shape,
    Edge,
};

/// Control Flow Graph
pub struct ControlFlowGraph<'a> {
    edges: HashSet<Edge>,
    vertices: HashSet<Vertex>,
    dict: &'a Dictionary<'a>,
    start: u32,
    stop: u32,
}

/// The type of breaking loop statement
#[derive(Debug, PartialEq)]
pub enum BreakerType {
    Continue,
    Break,
}

/// Type and position of `LoopBreaker`
#[derive(Debug)]
pub struct LoopBreaker {
    kind: BreakerType,
    id: u32,
}

impl<'a> ControlFlowGraph<'a> {
    /// Create a new cfg from dictionary
    pub fn new(dict: &'a Dictionary, entry_id: u32) -> Self {
        let mut cfg = ControlFlowGraph {
            edges: HashSet::new(),
            vertices: HashSet::new(),
            dict,
            start: 0,
            stop: 0,
        };
        cfg.start_at(entry_id);
        cfg
    }

    pub fn get_start(&self) -> u32 {
        self.start
    }

    pub fn get_stop(&self) -> u32 {
        self.stop
    }

    pub fn get_dict(&self) -> &Dictionary {
        self.dict
    }

    pub fn get_vertices(&self) -> &HashSet<Vertex> {
        &self.vertices
    }

    pub fn get_edges(&self) -> &HashSet<Edge> {
        &self.edges
    }

    /// Traverse comparison nodes in IfStatement, WhileStatement, DoWhileStatement 
    ///
    /// Build a list of nested function calls and connect them toghether
    pub fn condition_traverse(&mut self, blocks: &Vec<SimpleBlockNode>) -> Vec<u32> {
        let mut chains = vec![];
        for block in blocks {
            match block {
                SimpleBlockNode::FunctionCall(walker) => {
                    let Node { id, source, .. } = walker.node;
                    let vertice = Vertex::new(id, source, Shape::DoubleCircle);
                    self.vertices.insert(vertice);
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
            let edge = Edge::new(chains[index], chains[index + 1]);
            self.edges.insert(edge);
        }
        chains
    }

    /// Traverse a list of SimpleBlockNode
    pub fn simple_traverse(&mut self, blocks: &Vec<SimpleBlockNode>, mut predecessors: Vec<u32>, breakers: &mut Vec<LoopBreaker>) -> Vec<u32> {
        for block in blocks.iter() {
            if predecessors.is_empty() { return vec![]; }
            match block {
                SimpleBlockNode::Break(walker) => {
                    let Node { id, source, .. } = walker.node;
                    let vertice = Vertex::new(id, source, Shape::Box);
                    self.vertices.insert(vertice);
                    for predecessor in predecessors.iter() {
                        let edge = Edge::new(*predecessor, id);
                        self.edges.insert(edge);
                    }
                    breakers.push(LoopBreaker { kind: BreakerType::Break, id });
                    predecessors = vec![];
                },
                SimpleBlockNode::Continue(walker) => {
                    let Node { id, source, .. } = walker.node;
                    let vertice = Vertex::new(id, source, Shape::Box);
                    self.vertices.insert(vertice);
                    for predecessor in predecessors.iter() {
                        let edge = Edge::new(*predecessor, id);
                        self.edges.insert(edge);
                    }
                    breakers.push(LoopBreaker { kind: BreakerType::Continue, id });
                    predecessors = vec![];
                },
                SimpleBlockNode::Require(walker)
                    | SimpleBlockNode::Assert(walker)
                    | SimpleBlockNode::Transfer(walker) => {
                    let Node { id, source, .. } = walker.node;
                    let vertice = Vertex::new(id, source, Shape::DoubleCircle);
                    self.vertices.insert(vertice);
                    for predecessor in predecessors.iter() {
                        let edge = Edge::new(*predecessor, id);
                        self.edges.insert(edge);
                    }
                    let edge = Edge::new(id, self.stop);
                    self.edges.insert(edge);
                    predecessors = vec![id];
                },
                SimpleBlockNode::Throw(walker) => {
                    let Node { id, source, .. } = walker.node;
                    let vertice = Vertex::new(id, source, Shape::Box);
                    self.vertices.insert(vertice);
                    for predecessor in predecessors.iter() {
                        let edge = Edge::new(*predecessor, id);
                        self.edges.insert(edge);
                    }
                    let edge = Edge::new(id, self.stop);
                    self.edges.insert(edge);
                    predecessors = vec![];
                },
                SimpleBlockNode::Revert(walker) 
                    | SimpleBlockNode::Selfdestruct(walker)
                    | SimpleBlockNode::Suicide(walker) => {
                    let Node { id, source, .. } = walker.node;
                    let vertice = Vertex::new(id, source, Shape::DoubleCircle);
                    self.vertices.insert(vertice);
                    for predecessor in predecessors.iter() {
                        let edge = Edge::new(*predecessor, id);
                        self.edges.insert(edge);
                    }
                    let edge = Edge::new(id, self.stop);
                    self.edges.insert(edge);
                    predecessors = vec![];
                },
                SimpleBlockNode::FunctionCall(walker) | SimpleBlockNode::ModifierInvocation(walker) => {
                    let Node { id, source, .. } = walker.node;
                    predecessors = predecessors
                        .iter()
                        .filter_map(|predecessor| {
                            let edge = Edge::new(*predecessor, id);
                            if !self.edges.insert(edge) { return None; }
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
                            let edge = Edge::new(*predecessor, id);
                            if !self.edges.insert(edge) { return None; }
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

    /// Traverse the whole graph
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
                                        let edge = Edge::new(*predecessor, chains[0]);
                                        self.edges.insert(edge);
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
                                            let edge = Edge::new(*predecessor, chains[0]);
                                            self.edges.insert(edge);
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
                                        let edge = Edge::new(*predecessor, chains[0]);
                                        self.edges.insert(edge);
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
                                        let edge = Edge::new(*predecessor, chains[0]);
                                        self.edges.insert(edge);
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
                                            let edge = Edge::new(*predecessor, chains[0]);
                                            self.edges.insert(edge);
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
                            for predecessor in predecessors.iter() {
                                let edge = Edge::new(*predecessor, self.stop);
                                self.edges.insert(edge);
                            }
                            predecessors = vec![];
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

    /// Build a cfg, the cfg starts at FunctionDefinition, ModifierDefinition `entry_id`
    pub fn start_at(&mut self, entry_id: u32) {
        let walker = self.dict.lookup(entry_id).expect("must exist").clone();
        self.start = entry_id * 100000;
        self.stop = self.start + 1;
        match walker.node.name {
            "FunctionDefinition" | "ModifierDefinition" => {
                let mut graph = Graph::new(walker);
                let root = graph.update();
                let states = self.dict.lookup_states(LookupInputType::FunctionId(entry_id));
                if let BlockNode::Root(blocks) = root {
                    for id in vec![self.start, self.stop] {
                        let vertex = Vertex::new(id, "", Shape::Point);
                        self.vertices.insert(vertex);
                    }
                    let last_id = states.iter().fold(self.start, |prev, cur| {
                        let vertex = Vertex::new(cur.node.id, cur.node.source, Shape::Box);
                        let edge = Edge::new(prev, cur.node.id);
                        self.vertices.insert(vertex);
                        self.edges.insert(edge);
                        cur.node.id
                    });
                    let predecessors = self.traverse(blocks, vec![last_id], &mut vec![]);
                    for predecessor in predecessors.iter() {
                        let edge = Edge::new(*predecessor, self.stop);
                        self.edges.insert(edge);
                    }
                }
            },
            "ContractDefinition" => {
                let states = self.dict.lookup_states(LookupInputType::ContractId(entry_id));
                for id in vec![self.start, self.stop] {
                    let vertex = Vertex::new(id, "", Shape::Point);
                    self.vertices.insert(vertex);
                }
                let last_id = states.iter().fold(self.start, |prev, cur| {
                    let vertex = Vertex::new(cur.node.id, cur.node.source, Shape::Box);
                    let edge = Edge::new(prev, cur.node.id);
                    self.vertices.insert(vertex);
                    self.edges.insert(edge);
                    cur.node.id
                });
                let edge = Edge::new(last_id, self.stop);
                self.edges.insert(edge);
            },
            _ => {},
        }
    }

    /// Find all paths of current cfg
    /// from starting point to the end 
    pub fn find_execution_paths(&self, start_at: u32, paths: &mut Vec<Vec<u32>>) {
        if paths.is_empty() {
            paths.push(vec![start_at]);
        }
        let mut childs = vec![];
        for edge in self.edges.iter() {
            if edge.get_from() == start_at {
                childs.push(edge.get_to());
            }
        }
        if !childs.is_empty() {
            let mut is_extensible = false;
            let prev_paths = paths.clone();
            paths.clear();
            for path in prev_paths {
                let prev_path_len = paths.len();
                if path.last().unwrap() == &start_at {
                    for child in childs.iter() {
                        // path vector is stored or not 
                        if let Some(pos) = path.iter().position(|x| x == child) {
                            if path[pos - 1] != start_at {
                                let mut new_path = path.clone();
                                new_path.push(*child);
                                paths.push(new_path);
                                is_extensible = true;
                            }
                        } else {
                            let mut new_path = path.clone();
                            new_path.push(*child);
                            paths.push(new_path);
                            is_extensible = true;
                        }
                    }
                }
                if paths.len() == prev_path_len {
                    paths.push(path);
                }
            }
            if is_extensible {
                for child in childs {
                    self.find_execution_paths(child, paths);
                }
            }
        }
    }

}
