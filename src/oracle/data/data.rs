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
pub struct DataFlowGraph {
    parents: HashMap<u32, Vec<u32>>, 
    tables: HashMap<u32, FlowTable>,
    start: u32,
    stop: u32,
}

impl DataFlowGraph {
    pub fn new() -> Self {
        DataFlowGraph {
            parents: HashMap::new(),
            tables: HashMap::new(),
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
        for Vertex { id, ..} in vertices {
            self.tables.insert(*id, FlowTable::new());
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
}

impl Oracle for DataFlowGraph {
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
                    let variables = self.find_parameters(id, dict);
                    item = FlowItem::Variables(variables);
                },
                Shape::Box => {
                    let assignments = self.find_assignments(id, dict);
                    item = FlowItem::Assignments(assignments);
                },
                Shape::Diamond => {
                    item = FlowItem::Comparison;
                },
                Shape::Point => {
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
    }
}
