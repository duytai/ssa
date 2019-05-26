mod walker;
mod graph;
mod flow;
mod dict;
mod vertex;
mod analyzer;

use flow::{ ControlFlowGraph, GraphKind, GraphConfig };
use analyzer::{ 
    Dot, 
    DataFlowGraph,
    Analyzer,
};
use json;
use std::{
    fs,
    io,
    path::{ Path },
};

fn main() -> io::Result<()> {
    let home_dir = env!("CARGO_MANIFEST_DIR");
    let ast_file = Path::new(home_dir).join("assets/out.json");
    let source_file = Path::new(home_dir).join("assets/Identifier.sol");
    let ast_content = fs::read_to_string(ast_file)?;
    let source_content = fs::read_to_string(source_file)?;
    let ast_json = json::parse(&ast_content).expect("Invalid json format");
    for source in ast_json["sourceList"].members() {
        let source = source.as_str().unwrap();
        let ast_one = &ast_json["sources"][source]["AST"];
        let mut control_flow = ControlFlowGraph::new(ast_one, &source_content);
        let config = GraphConfig { 
            kind: GraphKind::Function("test"),
            contract_name: "Identifier",
            include_state: false,
        };
        let dot = Dot::new();
        let data_flow = DataFlowGraph::new();
        let handlers: Vec<Box<Analyzer>> = vec![
            Box::new(data_flow),
            Box::new(dot),
        ];
        control_flow.analyze(&config, handlers);
    }
    Ok(())
}
