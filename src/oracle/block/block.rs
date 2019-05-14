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

    pub fn find_assignments(&self, walker: &Walker) -> Vec<Assignment> {
        let mut assignments = vec![];
        walker.all(|walker| {
            walker.node.name == "Assignment"
        }, |walkers| {
            for walker in walkers {
                let operator = walker.node
                    .attributes["operator"]
                    .as_str()
                    .unwrap();
                let mut lhs = HashSet::new();
                let mut rhs = HashSet::new();
                walker.for_each(|walker, index| {
                    if index == 0 {
                        if let Some(variable) = Variable::parse(&walker) {
                            lhs.insert(variable);
                        }
                    } else if index == 1 {
                        walker.all_break(|walker| {
                            walker.node.name == "FunctionCall"
                            || walker.node.name == "Identifier"
                            || walker.node.name == "MemberAccess"
                            || walker.node.name == "IndexAccess"
                        }, |walkers| {
                            for walker in walkers {
                                if let Some(variable) = Variable::parse(&walker) {
                                    rhs.insert(variable);
                                }
                            }
                        })
                    }
                });
                let assignment = Assignment::new(lhs, rhs, operator);
                assignments.push(assignment);
            }
        });
        assignments
    }

    pub fn find_sending_variables(&self, walker: &Walker) -> HashSet<Variable> {
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
                                    let variable = Variable::parse(&walker);
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
        let mut stack = vec![self.stop];
        while stack.len() > 0 {
            let id = stack.pop().unwrap();
            if id != self.stop && id != self.start {
                let vertex = vertices.iter().find(|v| v.id == id).unwrap();
                let walker = dict.lookup(id).unwrap();
                match vertex.shape {
                    Shape::DoubleCircle => {
                        let variables = self.find_sending_variables(&walker);
                        let table = self.tables.get_mut(&id).unwrap();
                        let item = FlowItem::Variables(variables);
                        table.insert(item);
                    },
                    Shape::Box => {
                        let assignments = self.find_assignments(&walker);
                    },
                    Shape::Diamond => {},
                    Shape::Point => {},
                }
            }
            if let Some(parents) = self.parents.get_mut(&id) {
                stack.append(parents);
            }
        }
        println!("{:?}", self.tables);
    }
}
