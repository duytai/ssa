use std::collections::HashSet;
use crate::core::{ Variable, Assignment, Dictionary };

pub fn find_assignments(id: u32, dict: &Dictionary) -> Vec<Assignment> {
    dict.lookup(id)
        .map(|walker| Assignment::parse(walker, dict))
        .unwrap_or(vec![])
}

pub fn find_variables(id: u32, dict: &Dictionary) -> HashSet<Variable> {
    dict.lookup(id)
        .map(|walker| Variable::parse(walker, dict))
        .unwrap_or(HashSet::new())
}
