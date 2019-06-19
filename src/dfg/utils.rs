use std::collections::HashSet;
use crate::core::{ Variable, Assignment, Dictionary };

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
            for walker in &walker.direct_childs(|_| true)[1..] {
                let vars = Variable::parse(walker, dict);
                variables.extend(vars);
            }
            variables
        },
        None => HashSet::new(),
    }
}
