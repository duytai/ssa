/// TODO: handle function call
use std::collections::{ HashSet, HashMap };
use crate::{
    vertex::{ Vertex, Shape },
    dict::Dictionary,
    oracle::{ Oracle },
    walker::{ Walker },
};
use super::{
    variable::{ Variable },
    assignment::{ Assignment },
    table::{ FlowTable, FlowItem },
};

#[derive(Debug)]
pub struct BlockDependency {
    parents: HashMap<u32, Vec<u32>>, 
    tables: HashMap<u32, FlowTable>,
    start: u32,
    stop: u32,
}

impl BlockDependency {
    pub fn new() -> Self {
        BlockDependency {
            parents: HashMap::new(),
            tables: HashMap::new(),
            start: 0,
            stop: 1000000,
        }
    }

    pub fn initialize(&mut self, edges: &HashSet<(u32, u32)>, vertices: &HashSet<Vertex>) {
        for (from, to) in edges {
            match self.parents.get_mut(to) {
                Some(v) => {
                    v.push(*from);
                },
                None => {
                    self.parents.insert(*to, vec![*from]);
                }
            }
        }
        for Vertex { id, ..} in vertices {
            self.tables.insert(*id, FlowTable::new());
        }
    }

    pub fn find_assignments(&self,id: u32, dict: &Dictionary) -> Vec<Assignment> {
        let mut assignments = vec![];
        let mut declaration_walkers: Vec<Walker> = vec![];
        let walker = dict.lookup(id).unwrap();
        if walker.node.name == "VariableDeclarationStatement" {
            declaration_walkers.push(walker.clone());
        }
        walker.all(|walker| {
            walker.node.name == "Assignment"
        }, |mut walkers| {
            walkers.append(&mut declaration_walkers);
            for walker in walkers {
                let operator = walker.node
                    .attributes["operator"]
                    .as_str()
                    .unwrap_or("=");
                let mut lhs = HashSet::new();
                let mut rhs = HashSet::new();
                walker.for_all(|_| {
                    true
                }, |walkers| {
                    let id = walkers[0].node.id;
                    if let Some(variable) = Variable::parse(&walkers[0], dict) {
                        lhs.insert(variable);
                    }
                    walker.all_break(|walker| {
                        walker.node.name == "FunctionCall"
                        || walker.node.name == "Identifier"
                        || walker.node.name == "MemberAccess"
                        || walker.node.name == "IndexAccess"
                    }, |walkers| {
                        for walker in walkers {
                            if walker.node.id != id {
                                if let Some(variable) = Variable::parse(&walker, dict) {
                                    rhs.insert(variable);
                                }
                            }
                        }
                    });
                });
                let assignment = Assignment::new(lhs, rhs, operator);
                assignments.push(assignment);
            }
        });
        assignments
    }

    pub fn find_sending_variables(&self, id: u32, dict: &Dictionary) -> HashSet<Variable> {
        let walker = dict.lookup(id).unwrap();
        let mut variables = HashSet::new();
        walker.for_all(|_| {
            true
        }, |walkers| {
            if walkers.len() == 2 {
                if walkers[0].node.name == "MemberAccess" {
                    let id = walkers[0].node.id;
                    let no_ref = walkers[0]
                        .node
                        .attributes["referencedDeclaration"]
                        .is_null();
                    let member_name = walkers[0]
                        .node
                        .attributes["member_name"]
                        .as_str()
                        .unwrap();
                    if (member_name == "value"
                        || member_name == "send"
                        || member_name == "call"
                        || member_name == "transfer"
                        || member_name == "callcode") && no_ref {
                        walker.all_break(|walker| {
                            walker.node.name == "FunctionCall"
                            || walker.node.name == "Identifier"
                            || walker.node.name == "MemberAccess"
                            || walker.node.name == "IndexAccess"
                        }, |walkers| {
                            for walker in walkers {
                                if walker.node.id != id {
                                    let variable = Variable::parse(&walker, dict);
                                    if let Some(variable) = variable {
                                        variables.insert(variable);
                                    }
                                }
                            }
                        });
                    }
                }
            }
        });
        variables
    }
}

impl Oracle for BlockDependency {
    fn analyze(&mut self, edges: &HashSet<(u32, u32)>, vertices: &HashSet<Vertex>, dict: &Dictionary) {
        self.initialize(edges, vertices);
        let mut stack: Vec<(u32, u32)> = vec![];
        let mut visted: HashSet<u32> = HashSet::new();
        if let Some(parents) = self.parents.get_mut(&self.stop) {
            for parent in parents {
                stack.push((self.stop, *parent));
            }
            visted.insert(self.stop);
        }
        while stack.len() > 0 {
            let (child, id) = stack.pop().unwrap();
            let vertex = vertices.iter().find(|v| v.id == id).unwrap();
            let child = self.tables.get(&child).unwrap();
            let mut item;
            match vertex.shape {
                Shape::DoubleCircle => {
                    let variables = self.find_sending_variables(id, dict);
                    item = FlowItem::Variables(variables);
                },
                Shape::Box => {
                    let assignments = self.find_assignments(id, dict);
                    item = FlowItem::Assignments(assignments);
                },
                Shape::Diamond => {
                    // TODO
                    item = FlowItem::Comparison;
                },
                Shape::Point => {
                    // TODO
                    item = FlowItem::None;
                },
            }
            let table = FlowTable::merge(child, item);
            let cur = self.tables.get(&id).unwrap();
            if &table != cur || !visted.contains(&id) {
                self.tables.insert(id, table);
                if let Some(parents) = self.parents.get_mut(&id) {
                    for parent in parents {
                        stack.push((id, *parent));
                    }
                }
            }
            visted.insert(id);
        }
        for (id, table) in self.tables.iter() {
            println!("{} - {:?}", id, table);
        }
        let root = self.tables.get(&self.start).unwrap();
        if root.is_vulerable() {
            println!(">>>>");
            println!(">>>>VULNERABLE");
            println!(">>>>");
        }
    }
}
