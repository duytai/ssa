use crate::dot::edge::DotEdge;
use crate::dot::vertex::DotVertex;

pub struct Dot {
    edges: Vec<DotEdge>,
    vertices: Vec<DotVertex>,
}

impl Dot {
    pub fn new() -> Self {
        Dot { edges: vec![], vertices: vec![] }
    }

    pub fn add_vertex(&mut self, vertex: DotVertex) {
        self.vertices.push(vertex);
    }

    pub fn append_vertices(&mut self, vertices: Vec<DotVertex>) {
        for vertex in vertices {
            self.vertices.push(vertex);
        }
    }

    pub fn add_edge(&mut self, edge: DotEdge) {
        self.edges.push(edge);
    }

    pub fn append_edges(&mut self, edges: Vec<DotEdge>) {
        for edge in edges {
            self.edges.push(edge);
        }
    }

    pub fn format(&self) -> String {
        let mut edges = vec![];
        let mut vertices = vec![];
        for edge in self.edges.iter() {
            edges.push(format!("\t{}", edge.format()));
        } 
        for vertex in self.vertices.iter() {
            vertices.push(format!("\t{}", vertex.format()));
        }
        format!("digraph {{\n{0}\n{1}\n}}", edges.join("\n"), vertices.join("\n"))
    }
}
