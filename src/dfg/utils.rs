use std::collections::HashSet;
use crate::core::Dictionary;
use crate::dfg::{ Variable, Assignment };

pub fn find_assignments(id: u32, dict: &Dictionary) -> Vec<Assignment> {
    match dict.lookup(id) {
        Some(walker) => Assignment::parse(walker, dict),
        None => vec![],
    }
}

pub fn find_variables(id: u32, dict: &Dictionary) -> HashSet<Variable> {
    match dict.lookup(id) {
        Some(walker) => Variable::parse(walker, dict),
        None => HashSet::new(),
    }
}

pub fn find_parameters(id: u32, dict: &Dictionary) -> HashSet<Variable> {
    match dict.lookup(id) {
        Some(walker) => {
            let mut variables = HashSet::new();
            walker.for_all(|_| { true }, |walkers| {
                for walker in &walkers[1..] {
                    let vars = Variable::parse(walker, dict);
                    variables.extend(vars);
                }
            });
            variables
        },
        None => HashSet::new(),
    }
}
