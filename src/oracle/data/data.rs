use std::collections::{ HashSet, HashMap };
use crate::{
    vertex::{ Vertex, Shape },
    dict::Dictionary,
    oracle::{ Oracle },
    walker::{ Walker },
};
use super::{
    variable::{ Variable },
    assignment::{ Assignment, Operator },
};

#[derive(Debug)]
pub struct DataFlowGraph {
    parents: HashMap<u32, Vec<u32>>, 
    start: u32,
    stop: u32,
}

impl DataFlowGraph {
    pub fn new() -> Self {
        DataFlowGraph {
            parents: HashMap::new(),
            start: 0,
            stop: 1000000,
        }
    }

    pub fn initialize(&mut self, edges: &HashSet<(u32, u32)>, vertices: &HashSet<Vertex>) {
        for (from, to) in edges {
            match self.parents.get_mut(to) {
                Some(v) => { v.push(*from); },
                None => { self.parents.insert(*to, vec![*from]); },
            }
        }
    }

    pub fn find_assignments(&self,id: u32, dict: &Dictionary) -> Vec<Assignment> {
        let walker = dict.lookup(id).unwrap();
        Assignment::parse(walker, dict)
    }

    pub fn find_parameters(&self, id: u32, dict: &Dictionary) -> HashSet<Variable> {
        let walker = dict.lookup(id).unwrap();
        let mut variables = HashSet::new();
        walker.for_all(|_| { true }, |walkers| {
            for walker in &walkers[1..] {
                let vars = Variable::parse(walker, dict);
                variables.extend(vars);
            }
        });
        variables
    }

    pub fn traverse(
        &self,
        id: u32,
        visited: &mut HashSet<u32>,
        mut variables: HashMap<Variable, HashSet<u32>>,
        vertices: &HashSet<Vertex>,
        dict: &Dictionary
    ) {
        let vertex = vertices.iter().find(|v| v.id == id).unwrap();
        match vertex.shape {
            Shape::DoubleCircle => {
                let vars = self.find_parameters(id, dict);
                for var in vars {
                    if let Some(ids) = variables.get_mut(&var) {
                        ids.insert(id);
                    } else {
                        let mut ids = HashSet::new();
                        ids.insert(id);
                        variables.insert(var, ids);
                    }
                }
            },
            Shape::Box => {
                let assignments = self.find_assignments(id, dict);
                println!("len: {}", assignments.len());
                for assignment in assignments {
                    let Assignment { lhs, rhs, op } = assignment;
                    match op {
                        Operator::Equal => {},
                        Operator::Other => {},
                    }
                }
            },
            Shape::Diamond => {},
            Shape::Point => {},
        }
        visited.insert(id);
        println!("{} - variables: {:?}", id, variables);
        if let Some(parents) = self.parents.get(&id) {
            for parent in parents {
                self.traverse(*parent, visited, variables.clone(), vertices, dict);
            }
        }
    }
}

impl Oracle for DataFlowGraph {
    fn analyze(
        &mut self,
        edges: &HashSet<(u32, u32)>,
        vertices: &HashSet<Vertex>,
        dict: &Dictionary
    ) {
        self.initialize(edges, vertices);
        let mut visited: HashSet<u32> = HashSet::new();
        self.traverse(self.stop, &mut visited, HashMap::new(), vertices, dict);
    }
}
