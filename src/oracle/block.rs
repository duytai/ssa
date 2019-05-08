use std::collections::{ HashSet, HashMap };
use crate::{
    vertex::{ Vertex, Shape },
    dict::Dictionary,
    oracle::Oracle,
};

#[derive(Debug)]
pub struct Block {
    to_vertice: HashMap<u32, u32>, // child -> parent
}

impl Block {
    pub fn new() -> Self {
        Block { to_vertice: HashMap::new() }
    }

    pub fn build_childs(&mut self, vertices: &HashSet<Vertex>, dict: &Dictionary) {
        for Vertex { id, .. } in vertices {
            let walker = dict.lookup(*id);
            if let Some(walker) = walker {
                walker.all(|_| {
                    true
                }, |walkers| {
                    for walker in walkers {
                        self.to_vertice.insert(walker.node.id, *id);
                    }
                });
            }
        }
    }

    pub fn find_sending_identifiers(&mut self, vertices: &HashSet<Vertex>, dict: &Dictionary) -> HashSet<u32> {
        let mut variables_ids = HashSet::new();
        for Vertex { id, shape, .. } in vertices {
            if shape == &Shape::DoubleCircle {
                let walker = dict.lookup(*id).unwrap();
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
                                        let id = walkers[1]
                                            .node
                                            .attributes["referencedDeclaration"]
                                            .as_u32()
                                            .unwrap();
                                        variables_ids.insert(id);
                                    },
                                    _ => {
                                        walkers[1].all_break(|walker| {
                                            walker.node.name == "FunctionCall" || walker.node.name == "Identifier"
                                        }, |walkers| {
                                            for walker in walkers {
                                                match walker.node.name {
                                                    "Identifier" => {
                                                        let id = walker
                                                            .node
                                                            .attributes["referencedDeclaration"]
                                                            .as_u32()
                                                            .unwrap();
                                                        variables_ids.insert(id);
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
            } 
        }
        variables_ids.iter().map(|id| {
            self.to_vertice[id]
        }).collect::<HashSet<u32>>()
    }
}

impl Oracle for Block {
    fn analyze(&mut self, edges: &HashSet<(u32, u32)>, vertices: &HashSet<Vertex>, dict: &Dictionary) {
        self.build_childs(vertices, dict);
        let identifiers = self.find_sending_identifiers(vertices, dict);
        println!("identifiers: {:?}", identifiers);
    }
}
