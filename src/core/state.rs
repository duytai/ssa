use std::collections::HashSet;
use crate::core::{
    edge::Edge,
    vertex::{ Vertex },
    dict::{ Dictionary },
};

/// Result of CFG
#[derive(Debug)]
pub struct State<'a> {
    pub stop: u32,
    pub edges: &'a HashSet<Edge>,
    pub vertices: &'a HashSet<Vertex>,
    pub dict: &'a Dictionary<'a>, 
}

impl<'a> State<'a> {
    pub fn new(stop: u32, edges: &'a HashSet<Edge>, vertices: &'a HashSet<Vertex>, dict: &'a Dictionary<'a>) -> Self {
        State {
            stop,
            edges,
            vertices,
            dict,
        }
    }
}
