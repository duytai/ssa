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

    pub fn clear(&mut self) {
        self.edges.clear();
        self.vertices.clear();
        self.links.clear();
    }

    pub fn add_cfg(&mut self, cfg: &ControlFlowGraph) {
        for edge in cfg.get_edges().iter() {
            self.edges.push(format!("  {} -> {};", edge.get_from(), edge.get_to()));
        }
        for (id, vertex) in cfg.get_vertices().iter() {
            let source = vertex.get_source().replace("\"", "");
            let shape = match vertex.get_shape() {
                Shape::Entry => "point",
                Shape::Statement => "box",
                Shape::RootCondition => "diamond",
                Shape::FunctionCall => "doublecircle",
                Shape::Require => "doublecircle",
                Shape::Assert => "doublecircle",
                Shape::IndexAccess => "doublecircle",
                Shape::ConditionAndFunctionCall=> "doublecircle",
                Shape::ConditionAndIndexAccess => "doublecircle",
            };
            self.vertices.push(format!("  {}[label=\"{}\", shape=\"{}\"];", id, source, shape));
        }
    }

    pub fn add_links(&mut self, links: &HashSet<DataLink>) {
        for link in links.iter() {
            let from = link.get_from();
            let to = link.get_to();
            let from_var_label = from.0.get_source().replace("\"", "");
            let to_var_label = to.0.get_source().replace("\"", "");
            let label = format!("({}, {})", from_var_label, to_var_label);
            self.links.push(format!("  {} -> {}[label=\"{}\", style=dotted];", from.1, to.1, label));
        }
    }

    pub fn format(&self) -> String {
        format!("digraph {{\n{0}\n{1}\n{2}\n}}", self.edges.join("\n"), self.vertices.join("\n"), self.links.join("\n"))
    }
}
