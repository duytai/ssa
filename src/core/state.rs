use std::collections::HashSet;
use crate::core::{
    Edge,
    Vertex,
    Dictionary,
};

/// Result of CFG
#[derive(Debug)]
pub struct State<'a> {
    pub start: u32,
    pub stop: u32,
    pub edges: &'a HashSet<Edge>,
    pub vertices: &'a HashSet<Vertex>,
    pub dict: &'a Dictionary<'a>, 
}
