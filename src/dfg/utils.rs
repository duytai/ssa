use std::collections::HashSet;
use crate::core::{
    Variable,
    Assignment,
    Dictionary,
    Declaration,
    FunctionUse,
    IndexUse,
};

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

pub fn find_function_use(id: u32, dict: &Dictionary) -> Vec<FunctionUse> {
    dict.lookup(id)
        .map(|walker| FunctionUse::parse(walker, dict))
        .unwrap_or(vec![])
}

pub fn find_index_use(id: u32, dict: &Dictionary) -> Vec<IndexUse> {
    dict.lookup(id)
        .map(|walker| IndexUse::parse(walker, dict))
        .unwrap_or(vec![])
}
