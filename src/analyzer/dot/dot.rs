use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use crate::{
    analyzer::{ Analyzer, State },
};

pub struct Dot {
    file_path: PathBuf,
}

impl Dot {
    pub fn new(file_path: PathBuf) -> Self {
        Dot { file_path }
    }
}

impl Analyzer for Dot {
    fn analyze(&mut self, state: &mut State) {
        let mut vertices_str = String::from("");
        let mut edges_str = String::from("");
        let State { edges, vertices, links, .. } =  state;
        for edge in edges.iter() {
            let edge_str = format!("  {} -> {};\n", edge.0, edge.1);
            edges_str.push_str(&edge_str);
        }
        for vertice in vertices.iter() {
            vertices_str.push_str(&vertice.to_string());
        }
        if let Some(links) = links {
            for link in links.iter() {
                let label = &link.var.source;
                let edge_str = format!("  {} -> {}[label=\"{}\", style=dotted];\n", link.from, link.to, label);
                edges_str.push_str(&edge_str);
            }
        }

        let mut file = File::create(&self.file_path).unwrap();
        let data = format!("digraph {{\n{0}{1}}}", vertices_str, edges_str);
        println!("{}", data);
        file.write_all(data.as_bytes()).unwrap();
    }
}

