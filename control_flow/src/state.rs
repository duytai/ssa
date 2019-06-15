use std::collections::HashSet;
use crate::{
    vertex::{ Vertex },
    dict::{ Dictionary },
};

#[derive(Debug)]
pub struct State<'a> {
    pub stop: u32,
    pub edges: &'a HashSet<(u32, u32)>,
    pub vertices: &'a HashSet<Vertex>,
    pub dict: &'a Dictionary<'a>, 
}

impl<'a> State<'a> {
    pub fn new(stop: u32, edges: &'a HashSet<(u32, u32)>, vertices: &'a HashSet<Vertex>, dict: &'a Dictionary<'a>) -> Self {
        State {
            stop,
            edges,
            vertices,
            dict,
        }
    }
}
