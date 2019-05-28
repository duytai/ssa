use std::collections::HashSet;
use crate::{
    vertex::{ Vertex },
    dict::{ Dictionary },
};
use super::flow::{ DataLink };

#[derive(Debug)]
pub struct State<'a> {
    pub stop: u32,
    pub edges: &'a HashSet<(u32, u32)>,
    pub vertices: &'a HashSet<Vertex>,
    pub dict: &'a Dictionary<'a>, 
    pub links: Option<HashSet<DataLink>>,
}

pub trait Analyzer {
    fn analyze(&mut self, state: &mut State);
}
