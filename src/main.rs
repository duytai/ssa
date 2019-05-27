mod walker;
mod graph;
mod control_flow;
mod dict;
mod vertex;
mod analyzer;

use control_flow::{ ControlFlowGraph };
use dict::{ Dictionary };
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
        let dict = Dictionary::new(ast_one, &source_content);
        let mut control_flow = ControlFlowGraph::new(&dict);
        let handlers: Vec<Box<Analyzer>> = vec![
            Box::new(DataFlowGraph::new()),
            Box::new(Dot::new()),
        ];
        control_flow.analyze(31, handlers);
    }
    Ok(())
}
