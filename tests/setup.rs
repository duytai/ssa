use std::io::*;
use std::path::Path;
use ssa:: {
    core::{ Dictionary, State },
    cfg::{ ControlFlowGraph },
    dfg::{ DataFlowGraph },
    loader::{
        Solidity,
        SolidityOption,
        SolidityOutputKind,
        SolidityOutput,
        SolidityASTOutput,
    },
};

pub fn setup_cfg<T>(name: &str, entry_id: u32, mut cb: T) -> Result<()> where T: FnMut(State) {
    let home_dir = env!("CARGO_MANIFEST_DIR");
    let tests_dir = Path::new(home_dir).join("tests/");
    let contract_dir = tests_dir.join("contracts/");
    let option = SolidityOption {
        bin_dir: &tests_dir.join("bin/"),
        contract: &contract_dir.join(name),
        kind: SolidityOutputKind::AST,
    };
    let solidity = Solidity::new(option);
    let solidity_output = solidity.compile()?;
    match solidity_output {
        SolidityOutput::AST(SolidityASTOutput { ast, sources }) => {
            let ast_json = json::parse(&ast).expect("Invalid json format");
            let dict = Dictionary::new(&ast_json, &sources);
            let mut control_flow = ControlFlowGraph::new(&dict);
            let state = control_flow.start_at(entry_id).unwrap();
            cb(state)
        }
    }
    Ok(())
} 
