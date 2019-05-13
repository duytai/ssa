mod walker;
mod graph;
mod flow;
mod dict;
mod vertex;
mod oracle;

use flow::{ Flow, GraphKind, GraphConfig };
use oracle::{ 
    Dot, 
    BlockDependency, 
    Oracle,
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
        let mut flow = Flow::new(ast_one, &source_content);
        let config = GraphConfig { 
            kind: GraphKind::Function("test"),
            contract_name: "Identifier",
            include_state: true,
        };
        let dot = Dot::new();
        let block_dependency = BlockDependency::new();
        let mut handlers: Vec<Box<Oracle>> = vec![
            Box::new(dot),
            Box::new(block_dependency)
        ];
        flow.analyze(&config, handlers);
    }
    Ok(())
}
