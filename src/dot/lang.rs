use std::collections::HashSet;
use crate::cfg::ControlFlowGraph;
use crate::core::{
    DataLink,
    Shape,
};

pub struct Dot {}

impl Dot {
    pub fn format(cfg: &ControlFlowGraph, ls: &HashSet<DataLink>) -> String {
        let mut edges = vec![];
        let mut vertices = vec![];
        let mut links = vec![];
        for link in ls.iter() {
            let from = link.get_from();
            let to = link.get_to();
            let label = link.get_var().get_source();
            links.push(format!("  {} -> {}[label=\"{}\", style=dotted];", from, to, label));
        }
        for edge in cfg.get_edges().iter() {
            edges.push(format!("  {} -> {};", edge.get_from(), edge.get_to()));
        } 
        for vertex in cfg.get_vertices().iter() {
            let id = vertex.get_id();
            let source = vertex.get_source();
            let shape = match vertex.get_shape() {
                Shape::Point => "point",
                Shape::Box => "box",
                Shape::Diamond => "diamond",
                Shape::DoubleCircle => "doublecircle",
                Shape::Mdiamond => "Mdiamond",
            };
            vertices.push(format!("  {}[label={:?}, shape=\"{}\"];", id, source, shape));
        }
        format!("digraph {{\n{0}\n{1}\n{2}\n}}", edges.join("\n"), vertices.join("\n"), links.join("\n"))
    }
}
