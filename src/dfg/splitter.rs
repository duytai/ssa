use std::collections::HashSet;
use crate::core::{ Variable, Assignment, Dictionary };

/// Split a block of code into mutiple elems
///
/// Assignment
/// Variable
pub struct Splitter {
    visted_nodes: HashSet<u32>,
}

impl Splitter {
    pub fn new() -> Self {
        Splitter { visted_nodes: HashSet::new() }
    }

    pub fn find_assignments(&mut self, id: u32, dict: &Dictionary) -> Vec<Assignment> {
        dict.lookup(id)
            .map(|walker| Assignment::parse(walker, dict, &mut self.visted_nodes))
            .unwrap_or(vec![])
    }

    pub fn find_variables(&mut self, id: u32, dict: &Dictionary) -> HashSet<Variable> {
        dict.lookup(id)
            .map(|walker| Variable::parse(walker, dict, &mut self.visted_nodes))
            .unwrap_or(HashSet::new())
    }

    pub fn find_parameters(&mut self, id: u32, dict: &Dictionary) -> HashSet<Variable> {
        dict.lookup(id)
            .map(|walker| {
                let mut variables = HashSet::new();
                for walker in &walker.direct_childs(|_| true)[1..] {
                    let vars = Variable::parse(walker, dict, &mut self.visted_nodes);
                    variables.extend(vars);
                }
                variables
            })
        .unwrap_or(HashSet::new())
    }
}
