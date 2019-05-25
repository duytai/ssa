use std::collections::HashSet;
use crate::{
    vertex::{ Vertex },
    dict::{ Dictionary },
};

#[derive(Debug)]
pub struct State<'a> {
    pub edges: &'a HashSet<(u32, u32)>,
    pub vertices: &'a HashSet<Vertex>,
    pub dict: &'a Dictionary<'a>, 
}

pub trait Analyzer {
    fn analyze(&mut self, state: &State);
}
