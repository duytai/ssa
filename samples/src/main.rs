extern crate loader;
extern crate json;
extern crate core;
extern crate cfg;
extern crate dot;

use std::io::*;
use std::path::Path;
use core::{ Dictionary, State };
use cfg::{ ControlFlowGraph };
use dfg::{ DataFlowGraph };
use dot::{
    Dot,
    DotVertex,
    DotEdge,
};
use loader::{
    Solidity,
    SolidityOption,
    SolidityOutputKind,
    SolidityOutput,
    SolidityASTOutput,
};

fn main() -> Result<()> {
    let home_dir = env!("CARGO_MANIFEST_DIR");
    let assets_dir = Path::new(home_dir).join("assets/"); 
    let contract_dir = assets_dir.join("contracts/");
    let option = SolidityOption {
        bin_dir: &assets_dir.join("bin/"),
        contract: &contract_dir.join("Math.sol"),
        kind: SolidityOutputKind::AST,
    };
    let solidity = Solidity::new(option);
    let solidity_output = solidity.compile()?;
    match solidity_output {
        SolidityOutput::AST(SolidityASTOutput { ast, sources }) => {
            let ast_json = json::parse(&ast).expect("Invalid json format");
            let dict = Dictionary::new(&ast_json, &sources);
            let mut control_flow = ControlFlowGraph::new(&dict);
            let state = control_flow.start_at(19).unwrap();
            let State { vertices, edges, .. } = state;
            let data_flow = DataFlowGraph::new(&state);
            let links = data_flow.find_links();
            let mut dot = Dot::new();
            for vertex in vertices {
                let dot_vertex: DotVertex = vertex.to_tuple().into();
                dot.add_vertex(dot_vertex);
            }
            for edge in edges {
                let dot_edge: DotEdge = edge.to_tuple().into();
                dot.add_edge(dot_edge);
            }
            for link in links {
                let (from, to, var) = link.to_tuple();
                let (_, source) = var.to_tuple();
                let dot_edge: DotEdge = (from, to, source.clone()).into();
                dot.add_edge(dot_edge);
            }
            println!("{}", dot.format());
        }
    }
    Ok(())
}
