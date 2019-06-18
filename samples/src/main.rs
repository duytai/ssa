use std::io::*;
use std::path::Path;
use ssa:: {
    core::{ Dictionary },
    cfg::{ ControlFlowGraph },
    dfg::{ DataFlowGraph },
    dot::{
        Dot,
        DotVertex,
        DotEdge,
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
        contract: &contract_dir.join("Math.sol"),
        kind: SolidityOutputKind::AST,
    };
    let solidity = Solidity::new(option);
    let solidity_output = solidity.compile()?;
    match solidity_output {
        SolidityOutput::AST(SolidityASTOutput { ast, sources }) => {
            let ast_json = json::parse(&ast).expect("Invalid json format");
            let dict = Dictionary::new(&ast_json, &sources);
            // Create control flow graph
            let mut control_flow = ControlFlowGraph::new(&dict);
            let state = control_flow.start_at(19).unwrap();
            // Create data flow graph
            let data_flow = DataFlowGraph::new(&state);
            let links = data_flow.find_links();
            // Render in dot language
            let mut dot = Dot::new();
            let dot_vertices: Vec<DotVertex> = state.vertices.iter()
                .map(|vertex| vertex.to_tuple().into())
                .collect();
            let dot_edges: Vec<DotEdge> = state.edges.iter()
                .map(|edge| edge.to_tuple().into())
                .collect();
            let dot_links: Vec<DotEdge> = links.iter()
                .map(|link| {
                    let (from, to, var) = link.to_tuple();
                    let (_, source) = var.to_tuple();
                    (from, to, source.clone()).into()
                })
                .collect();
            dot.append_edges(dot_edges);
            dot.append_edges(dot_links);
            dot.append_vertices(dot_vertices);
            println!("{}", dot.format());
        }
    }
    Ok(())
}
