use std::collections::HashSet;
use std::collections::HashMap;
use crate::cfg::{
    Graph,
    BlockNode,
    SimpleBlockNode,
    CodeBlock,
    IfStatement,
    WhileStatement,
    DoWhileStatement,
    ForStatement,
    ReturnStatement,
    Splitter,
};
use crate::core::{
    SmartContractQuery,
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
    function_id: u32,
    execution_paths: Vec<Vec<u32>>,
    indexes: HashMap<u32, Vec<u32>>,
    fcalls: HashMap<u32, Vec<u32>>,
    returns: HashMap<u32, Vec<u32>>,
    parameters: HashMap<u32, Vec<u32>>,
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
    pub fn new(dict: &'a Dictionary, contract_id: u32, function_id: u32) -> Self {
        let mut cfg = ControlFlowGraph {
            edges: HashSet::new(),
            vertices: HashSet::new(),
            execution_paths: vec![],
            indexes: HashMap::new(),
            fcalls: HashMap::new(),
            returns: HashMap::new(),
            parameters: HashMap::new(),
            dict,
            start: 0,
            stop: 0,
            function_id: 0,
        };
        cfg.start_at(contract_id, function_id);
        cfg.update_execution_paths(cfg.start, vec![]);
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

    pub fn get_execution_paths(&self) -> &Vec<Vec<u32>> {
        &self.execution_paths
    }

    pub fn get_vertices(&self) -> HashMap<u32, Vertex> {
        let mut h: HashMap<u32, Vertex> = HashMap::new();
        for v in self.vertices.iter() {
            h.insert(v.get_id(), v.clone());
        }
        h
    }

    pub fn get_edges(&self) -> &HashSet<Edge> {
        &self.edges
    }

    pub fn get_indexes(&self) -> &HashMap<u32, Vec<u32>> {
        &self.indexes
    }

    pub fn get_fcalls(&self) -> &HashMap<u32, Vec<u32>> {
        &self.fcalls
    }

    pub fn get_returns(&self) -> &HashMap<u32, Vec<u32>> {
        &self.returns
    }

    pub fn get_parameters(&self) -> &HashMap<u32, Vec<u32>> {
        &self.parameters
    }

    /// Traverse comparison nodes in IfStatement, WhileStatement, DoWhileStatement
    ///
    /// Build a list of nested function calls and connect them toghether
    pub fn condition_traverse(&mut self, blocks: &Vec<SimpleBlockNode>, level: u32) -> Vec<u32> {
        let mut chains = vec![];
        for block in blocks {
            match block {
                SimpleBlockNode::FunctionCall(walker) => {
                    let Node { id, source, .. } = walker.node;
                    let vertice = Vertex::new(id, source, Shape::ConditionAndFunctionCall, level);
                    self.vertices.insert(vertice);
                    chains.push(id);
                },
                SimpleBlockNode::IndexAccess(walker) => {
                    let Node { id, source, .. } = walker.node;
                    let vertice = Vertex::new(id, source, Shape::ConditionAndIndexAccess, level);
                    self.vertices.insert(vertice);
                    chains.push(id);
                },
                SimpleBlockNode::Unit(walker) => {
                    let Node { id, source, .. } = walker.node;
                    let vertice = Vertex::new(id, source, Shape::Condition, level);
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
    pub fn simple_traverse(&mut self, blocks: &Vec<SimpleBlockNode>, mut predecessors: Vec<u32>, breakers: &mut Vec<LoopBreaker>, level: u32) -> Vec<u32> {
        for block in blocks.iter() {
            if predecessors.is_empty() { return vec![]; }
            match block {
                SimpleBlockNode::Break(walker) => {
                    let Node { id, source, .. } = walker.node;
                    let vertice = Vertex::new(id, source, Shape::Statement, level);
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
                    let vertice = Vertex::new(id, source, Shape::Statement, level);
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
                    let vertice = Vertex::new(id, source, Shape::FunctionCall, level);
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
                    let vertice = Vertex::new(id, source, Shape::Statement, level);
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
                    let vertice = Vertex::new(id, source, Shape::FunctionCall, level);
                    self.vertices.insert(vertice);
                    for predecessor in predecessors.iter() {
                        let edge = Edge::new(*predecessor, id);
                        self.edges.insert(edge);
                    }
                    let edge = Edge::new(id, self.stop);
                    self.edges.insert(edge);
                    predecessors = vec![];
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
                        let vertice = Vertex::new(id, source, Shape::Statement, level);
                        self.vertices.insert(vertice);
                    }
                    predecessors.dedup();
                },
                SimpleBlockNode::FunctionCall(walker)
                    | SimpleBlockNode::ModifierInvocation(walker)
                    | SimpleBlockNode::IndexAccess(walker) => {
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
                        let vertice = Vertex::new(id, source, Shape::FunctionCall, level);
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
    pub fn traverse(&mut self, blocks: &Vec<CodeBlock>, mut predecessors: Vec<u32>, breakers: &mut Vec<LoopBreaker>, level: u32) -> Vec<u32> {
        for block in blocks {
            if predecessors.is_empty() { return vec![]; }
            match block {
                CodeBlock::Block(walker) => {
                    let mut splitter = Splitter::new();
                    let simple_blocks = splitter.split(walker.clone());
                    self.indexes.extend(splitter.get_indexes().clone());
                    self.fcalls.extend(splitter.get_fcalls().clone());
                    predecessors = self.simple_traverse(&simple_blocks, predecessors.clone(), breakers, level);
                },
                CodeBlock::Link(link) => {
                    match &**link {
                        BlockNode::IfStatement(IfStatement { condition, tblocks, fblocks }) => {
                            if let CodeBlock::Block(walker) = condition {
                                let mut splitter = Splitter::new();
                                let condition_blocks = splitter.split(walker.clone());
                                self.indexes.extend(splitter.get_indexes().clone());
                                self.fcalls.extend(splitter.get_fcalls().clone());
                                let chains = self.condition_traverse(&condition_blocks, level);
                                if !chains.is_empty() {
                                    for predecessor in predecessors.iter() {
                                        let edge = Edge::new(*predecessor, chains[0]);
                                        self.edges.insert(edge);
                                    }
                                    predecessors = vec![chains[chains.len() - 1]];
                                    let mut t = self.traverse(tblocks, predecessors.clone(), breakers, level + 1);
                                    let mut f = self.traverse(fblocks, predecessors.clone(), breakers, level + 1);
                                    predecessors.clear();
                                    predecessors.append(&mut t);
                                    predecessors.append(&mut f);
                                }
                            }
                        },
                        BlockNode::DoWhileStatement(DoWhileStatement { condition, blocks }) => {
                            if let CodeBlock::Block(walker) = condition {
                                let mut our_breakers = vec![];
                                predecessors = self.traverse(blocks, predecessors.clone(), &mut our_breakers, level + 1);
                                our_breakers
                                    .iter()
                                    .filter(|breaker| breaker.kind == BreakerType::Continue)
                                    .for_each(|LoopBreaker { id, .. }| {
                                        predecessors.push(*id);
                                    });
                                if !predecessors.is_empty() {
                                    let mut splitter = Splitter::new();
                                    let condition_blocks = splitter.split(walker.clone());
                                    self.indexes.extend(splitter.get_indexes().clone());
                                    self.fcalls.extend(splitter.get_fcalls().clone());
                                    let chains = self.condition_traverse(&condition_blocks, level);
                                    if !chains.is_empty() {
                                        for predecessor in predecessors.iter() {
                                            let edge = Edge::new(*predecessor, chains[0]);
                                            self.edges.insert(edge);
                                        }
                                    }
                                    predecessors = vec![chains[chains.len() - 1]];
                                    self.traverse(blocks, predecessors.clone(), &mut our_breakers, level + 1);
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
                                let mut splitter = Splitter::new();
                                let condition_blocks = splitter.split(walker.clone());
                                self.indexes.extend(splitter.get_indexes().clone());
                                self.fcalls.extend(splitter.get_fcalls().clone());
                                let chains = self.condition_traverse(&condition_blocks, level);
                                if !chains.is_empty() {
                                    for predecessor in predecessors.iter() {
                                        let edge = Edge::new(*predecessor, chains[0]);
                                        self.edges.insert(edge);
                                    }
                                    predecessors = vec![chains[chains.len() - 1]];
                                    predecessors = self.traverse(blocks, predecessors.clone(), &mut our_breakers, level + 1);
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
                                let mut splitter = Splitter::new();
                                let simple_blocks = splitter.split(walker.clone());
                                self.indexes.extend(splitter.get_indexes().clone());
                                self.fcalls.extend(splitter.get_fcalls().clone());
                                predecessors = self.simple_traverse(&simple_blocks, predecessors.clone(), breakers, level);
                            }
                            for _ in 0..2 {
                                if let CodeBlock::Block(walker) = condition {
                                    let mut splitter = Splitter::new();
                                    let condition_blocks = splitter.split(walker.clone());
                                    self.indexes.extend(splitter.get_indexes().clone());
                                    self.fcalls.extend(splitter.get_fcalls().clone());
                                    let chains = self.condition_traverse(&condition_blocks, level);
                                    if !chains.is_empty() {
                                        for predecessor in predecessors.iter() {
                                            let edge = Edge::new(*predecessor, chains[0]);
                                            self.edges.insert(edge);
                                        }
                                        predecessors = vec![chains[chains.len() - 1]];
                                        cond_predecessors = vec![chains[chains.len() - 1]];
                                    }
                                }
                                predecessors = self.traverse(blocks, predecessors.clone(), &mut our_breakers, level + 1);
                                our_breakers
                                    .iter()
                                    .filter(|breaker| breaker.kind == BreakerType::Continue)
                                    .for_each(|LoopBreaker { id, .. }| {
                                        predecessors.push(*id);
                                    });
                                if let CodeBlock::Block(walker) = expression {
                                    let mut splitter = Splitter::new();
                                    let simple_blocks = splitter.split(walker.clone());
                                    self.indexes.extend(splitter.get_indexes().clone());
                                    self.fcalls.extend(splitter.get_fcalls().clone());
                                    predecessors = self.simple_traverse(&simple_blocks, predecessors.clone(), breakers, level);
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
                        BlockNode::ReturnStatement(ReturnStatement { body }) => {
                            if let CodeBlock::Block(walker) = body {
                                let mut splitter = Splitter::new();
                                let simple_blocks = splitter.split(walker.clone());
                                self.indexes.extend(splitter.get_indexes().clone());
                                self.fcalls.extend(splitter.get_fcalls().clone());
                                if let Some(returns) = self.returns.get_mut(&self.function_id) {
                                    returns.push(walker.node.id);
                                } else {
                                    self.returns.insert(self.function_id, vec![walker.node.id]);
                                }
                                predecessors = self.simple_traverse(&simple_blocks, predecessors.clone(), breakers, level);
                                for predecessor in predecessors.iter() {
                                    let edge = Edge::new(*predecessor, self.stop);
                                    self.edges.insert(edge);
                                }
                            }
                            predecessors = vec![];
                        },
                        BlockNode::Root(_) => unimplemented!(),
                        BlockNode::None => unimplemented!(),
                    }
                },
                CodeBlock::SimpleBlocks(blocks) => {
                    predecessors = self.simple_traverse(blocks, predecessors.clone(), breakers, level);
                },
                CodeBlock::None => unimplemented!(),
            }
        }
        return predecessors;
    }

    /// Build a cfg, the cfg starts at FunctionDefinition, ModifierDefinition `entry_id`
    pub fn start_at(&mut self, contract_id: u32, function_id: u32) {
        self.start = function_id * 100000;
        self.stop = self.start + 1;
        self.function_id = function_id;
        let mut graph = Graph::new(self.dict.walker_at(function_id).unwrap().clone());
        let root = graph.update();
        let states = self.dict.find_walkers(SmartContractQuery::StatesByContractId(contract_id));
        let level = 0;
        if let BlockNode::Root(blocks) = root {
            for id in vec![self.start, self.stop] {
                let vertex = Vertex::new(id, "", Shape::Entry, level);
                self.vertices.insert(vertex);
            }
            let last_id = states.iter().fold(self.start, |prev, cur| {
                let vertex = Vertex::new(cur.node.id, cur.node.source, Shape::Statement, level);
                let edge = Edge::new(prev, cur.node.id);
                self.vertices.insert(vertex);
                self.edges.insert(edge);
                cur.node.id
            });
            let predecessors = self.traverse(blocks, vec![last_id], &mut vec![], level + 1);
            for predecessor in predecessors.iter() {
                let edge = Edge::new(*predecessor, self.stop);
                self.edges.insert(edge);
            }
        }
        let parameters = graph.get_parameters().clone();
        self.parameters.insert(function_id, parameters);
    }

    fn update_execution_paths(&mut self, from: u32, mut execution_path: Vec<u32>) {
        if from == self.stop {
            execution_path.push(from);
            self.execution_paths.push(execution_path.clone());
        } else {
            let mut next_edges = vec![];
            let num_dups = execution_path.iter()
                .filter(|x| *x == &from)
                .count();
            if num_dups < 2 {
                execution_path.push(from);
                for edge in self.edges.iter() {
                    if edge.get_from() == from {
                        next_edges.push(edge.get_to());
                    }
                }
                for next_edge in next_edges {
                    self.update_execution_paths(next_edge, execution_path.clone());
                }
            }
        }
    }
}
