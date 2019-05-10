use std::collections::{ HashSet, HashMap };
use crate::{
    vertex::{ Vertex, Shape },
    dict::Dictionary,
    oracle::Oracle,
    walker::{ Walker },
};

#[derive(Debug)]
pub struct BlockDependency {
    parents: HashMap<u32, Vec<u32>>, 
    props: HashMap<u32, HashMap<u32, bool>>, // vertex_id -> (id -> kill or not)
    start: u32,
    stop: u32,
}

impl BlockDependency {
    pub fn new() -> Self {
        BlockDependency {
            parents: HashMap::new(),
            props: HashMap::new(),
            start: 0,
            stop: 1000000,
        }
    }

    pub fn initialize(&mut self, edges: &HashSet<(u32, u32)>) {
        for (from, to) in edges {
            match self.parents.get_mut(to) {
                Some(v) => {
                    v.push(*from);
                },
                None => {
                    self.parents.insert(*to, vec![*from]);
                    if to != &self.stop {
                        self.props.insert(*to, HashMap::new());
                    }
                }
            }
        }
    }

    pub fn find_assigment_identifiers(&self, walker: &Walker) -> HashSet<u32> {
        let mut identifiers = HashSet::new();
        identifiers
    }

    pub fn find_sending_identifiers(&self, walker: &Walker) -> HashSet<u32> {
        let mut identifiers = HashSet::new();
        walker.for_all(|_| {
            true
        }, |walkers| {
            if walkers.len() == 2 {
                if walkers[0].node.name == "MemberAccess" {
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
                        match walkers[1].node.name {
                            "Identifier" => {
                                let identifier = walkers[1]
                                    .node
                                    .attributes["referencedDeclaration"]
                                    .as_u32()
                                    .unwrap();
                                identifiers.insert(identifier);
                            },
                            _ => {
                                walkers[1].all_break(|walker| {
                                    walker.node.name == "FunctionCall" || walker.node.name == "Identifier"
                                }, |walkers| {
                                    for walker in walkers {
                                        match walker.node.name {
                                            "Identifier" => {
                                                let identifier = walker
                                                    .node
                                                    .attributes["referencedDeclaration"]
                                                    .as_u32()
                                                    .unwrap();
                                                identifiers.insert(identifier);
                                            },
                                            "FunctionCall" => {
                                                // TODO: Analyze function call
                                            },
                                            _ => panic!("Unexpected Match"),
                                        }
                                    }
                                });
                            }
                        }
                    }
                }
            }
        });
        identifiers
    }
}

impl Oracle for BlockDependency {
    fn analyze(&mut self, edges: &HashSet<(u32, u32)>, vertices: &HashSet<Vertex>, dict: &Dictionary) {
        self.initialize(edges);
        let mut stack = vec![self.stop];
        while stack.len() > 0 {
            let id = stack.pop().unwrap();
            if id != self.stop && id != self.start {
                let vertex = vertices.iter().find(|v| v.id == id).unwrap();
                let walker = dict.lookup(id).unwrap();
                match vertex.shape {
                    Shape::DoubleCircle => {
                        let sending_identifiers = self.find_sending_identifiers(walker);
                        let identifiers = self.props.get_mut(&id).unwrap();
                        for id in sending_identifiers {
                            if !identifiers.contains_key(&id) {
                                identifiers.insert(id, false);
                            }
                        }
                    },
                    Shape::Box => {
                        let assignment_identifiers = self.find_assigment_identifiers(walker);
                    },
                    Shape::Point => {},
                    Shape::Diamond => {},
                }
            }
            if let Some(parents) = self.parents.get_mut(&id) {
                stack.append(parents);
            }
        }
        println!("{:?}", self.props);
    }
}
