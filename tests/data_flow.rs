mod setup;

use std::io;
use setup::setup_cfg;
use ssa::dfg::DataFlowGraph;

#[test]
fn depend_on_state_variable() -> io::Result<()> {
    setup_cfg("data_flow_1.sol", 11, |state| {
        assert_eq!(state.vertices.len(), 5);
        assert_eq!(state.edges.len(), 4);
        let links = DataFlowGraph::new(&state).find_links();
        assert_eq!(links.len(), 1);
        for link in links {
            assert_eq!(link.get_from(), 9);
            assert_eq!(link.get_to(), 3);
        } 
    })?;
    Ok(())
}

#[test]
fn depend_on_parameters() -> io::Result<()> {
    setup_cfg("data_flow_2.sol", 11, |state| {
        assert_eq!(state.vertices.len(), 4);
        assert_eq!(state.edges.len(), 3);
        let links = DataFlowGraph::new(&state).find_links();
        assert_eq!(links.len(), 1);
        for link in links {
            assert_eq!(link.get_from(), 9);
            assert_eq!(link.get_to(), 4);
        } 
    })?;
    Ok(())
}

#[test]
fn depend_on_local_variables() -> io::Result<()> {
    setup_cfg("data_flow_3.sol", 12, |state| {
        assert_eq!(state.vertices.len(), 5);
        assert_eq!(state.edges.len(), 4);
        let links = DataFlowGraph::new(&state).find_links();
        assert_eq!(links.len(), 1);
        for link in links {
            assert_eq!(link.get_from(), 10);
            assert_eq!(link.get_to(), 8);
        } 
    })?;
    Ok(())
}
