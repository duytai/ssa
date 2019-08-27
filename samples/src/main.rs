use std::io::*;
use std::path::Path;
use ssa:: {
    core::{ Dictionary },
    oracle::{
        Oracle,
        OracleAction,
    },
    dfg::{
        Network,
    },
    loader::{
        Solidity,
        SolidityOption,
        SolidityOutputKind,
        SolidityOutput,
        SolidityASTOutput,
    },
};

fn main() -> Result<()> {
    let home_dir = env!("CARGO_MANIFEST_DIR");
    let assets_dir = Path::new(home_dir).join("assets/"); 
    let contract_dir = assets_dir.join("contracts/");
    let option = SolidityOption {
        bin_dir: &assets_dir.join("bin/"),
        contract: &contract_dir.join("Sample.sol"),
        kind: SolidityOutputKind::AST,
    };
    let solidity = Solidity::new(option);
    let solidity_output = solidity.compile()?;
    // ContractDefinitionNode of HumanStandardToken
    let entry = 362;
    match solidity_output {
        SolidityOutput::AST(SolidityASTOutput { ast, sources }) => {
            let ast_json = json::parse(&ast).expect("Invalid json format");
            let dict = Dictionary::new(&ast_json, &sources);
            let network = Network::new(&dict, entry);
            let mut oracle = Oracle::new(network);
            println!("{}", oracle.format());
            println!("==> Overflow Check");
            for (walker, reason) in oracle.run(OracleAction::IntegerOverflow) {
                println!("----");
                println!("exp\t: {}", walker.node.source);
                println!("reason\t: {}", reason);
            }
        }
    }
    Ok(())
}
