use std::collections::HashSet;
use crate::cfg::ControlFlowGraph;
use crate::core::{
    DataLink,
    Shape,
};

pub struct Dot {
    edges: Vec<String>,
    vertices: Vec<String>,
    links: Vec<String>,
}

impl Dot {
    pub fn new() -> Self {
        Dot { edges: vec![], vertices: vec![], links: vec![] }
    }

    pub fn add_cfg(&mut self, cfg: &ControlFlowGraph) {
        for edge in cfg.get_edges().iter() {
            self.edges.push(format!("  {} -> {};", edge.get_from(), edge.get_to()));
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
            self.vertices.push(format!("  {}[label={:?}, shape=\"{}\"];", id, source, shape));
        }
    }

    pub fn add_links(&mut self, links: &HashSet<DataLink>) {
        for link in links.iter() {
            let from = link.get_from();
            let to = link.get_to();
            let label = link.get_var().get_source();
            self.links.push(format!("  {} -> {}[label=\"{}\", style=dotted];", from, to, label));
        }
    }

    pub fn format(&self) -> String {
        format!("digraph {{\n{0}\n{1}\n{2}\n}}", self.edges.join("\n"), self.vertices.join("\n"), self.links.join("\n"))
    }
}
