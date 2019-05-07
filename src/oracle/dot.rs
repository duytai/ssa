use std::collections::HashSet;
use crate::{
    vertex::Vertex,
    dict::Dictionary
};

pub struct Dot {}

impl Dot {
    pub fn new(edges: &HashSet<(u32, u32)>, vertices: &HashSet<Vertex>, _dict: &Dictionary) {
        let mut vertices_str = String::from("");
        let mut edges_str = String::from("");
        for edge in edges.iter() {
            let edge_str = format!("  {} -> {};\n", edge.0, edge.1);
            edges_str.push_str(&edge_str);
        }
        for vertice in vertices.iter() {
            vertices_str.push_str(&vertice.to_string());
        }
        println!("digraph {{\n{0}{1}}}", vertices_str, edges_str);
    }
}

