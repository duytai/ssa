use std::collections::HashSet;
use crate::core::{ Variable, Assignment, Dictionary, Declaration };

pub fn find_declarations(id: u32, dict: &Dictionary) -> Vec<Declaration> {
    dict.lookup(id)
        .map(|walker| Declaration::parse(walker, dict))
        .unwrap_or(vec![])
}

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
