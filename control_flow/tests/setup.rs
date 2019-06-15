use std::fs;
use std::path::Path;
use std::process::{ Command, Output };
use std::io::{ Result, Error, ErrorKind };
use control_flow::{ ControlFlowGraph, Dictionary };
use json;

pub fn setup<T>(name: &str, mut cb: T)-> Result<()> where T: FnMut(ControlFlowGraph) {
    let home_dir = env!("CARGO_MANIFEST_DIR");
    let assets_dir = Path::new(home_dir).join("tests/assets");
    let source_path = assets_dir.join(name);
    let file_name = source_path.file_name().unwrap();
    let file_name = String::from(file_name.to_str().unwrap());
    let source_content = fs::read_to_string(&source_path)?;
    let output = Command::new("solc")
        .arg("--combined-json")
        .arg("ast")
        .arg(source_path)
        .output()?;
    let Output { status, stdout, stderr } = output;
    assert!(status.success());
    let ast_content = String::from_utf8(stdout).unwrap();
    let ast_json = json::parse(&ast_content).expect("Invalid json format");
    for source in ast_json["sourceList"].members() {
        let source = source.as_str().unwrap();
        let ast_one = &ast_json["sources"][source]["AST"];
        let dict = Dictionary::new(ast_one, &source_content);
        let mut control_flow = ControlFlowGraph::new(&dict);
        cb(control_flow);
    }
    Ok(())
}
