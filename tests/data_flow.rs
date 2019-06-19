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

#[test]
fn find_assignments() -> io::Result<()> {
    setup_cfg("data_flow_4.sol", 21, |state| {
        assert_eq!(state.vertices.len(), 7);
        assert_eq!(state.edges.len(), 6);
        let links = DataFlowGraph::new(&state).find_links();
        assert_eq!(links.len(), 3);
        for link in links {
            match link.get_from() {
                19 => assert_eq!(link.get_to(), 17),
                17 => assert_eq!(link.get_to(), 6),
                13 => assert_eq!(link.get_to(), 3),
                _ => assert!(false),
            }
        } 
    })?;
    Ok(())
}

#[test]
fn struct_assignments() -> io::Result<()> {
    setup_cfg("data_flow_5.sol", 52, |state| {
        assert_eq!(state.vertices.len(), 11);
        assert_eq!(state.edges.len(), 10);
        let links = DataFlowGraph::new(&state).find_links();
        assert_eq!(links.len(), 8);
        for link in links {
            match link.get_from() {
                22 => match link.get_to() {
                    13 | 7 => {},
                    _ => assert!(false),
                },
                50 => match link.get_to() {
                    7 | 44 => {},
                    _ => assert!(false),
                },
                38 => match link.get_to() {
                    32 | 7 => {},
                    _ => assert!(false),
                }
                32 => assert_eq!(link.get_to(), 13),
                28 => assert_eq!(link.get_to(), 16),
                _ => assert!(false),
            }
        }
    })?;
    Ok(())
}

#[test]
fn array_assignments() -> io::Result<()> {
    setup_cfg("data_flow_6.sol", 31, |state| {
        assert_eq!(state.vertices.len(), 7);
        assert_eq!(state.edges.len(), 6);
        let links = DataFlowGraph::new(&state).find_links();
        assert_eq!(links.len(), 4);
        for link in links {
            match link.get_from() {
                16 => match link.get_to() {
                    4 | 7 => {},
                    _ => assert!(false),
                },
                29 => match link.get_to() {
                    7 | 23 => {},
                    _ => assert!(false),
                },
                _ => assert!(false),
            }
        }
    })?;
    Ok(())
}

#[test]
fn variables_in_functioncall() -> io::Result<()> {
    setup_cfg("data_flow_7.sol", 46, |state| {
        assert_eq!(state.vertices.len(), 9);
        assert_eq!(state.edges.len(), 8);
        let links = DataFlowGraph::new(&state).find_links();
        assert_eq!(links.len(), 3);
        for link in links {
            match link.get_from() {
                40 => assert_eq!(26, link.get_to()),
                44 => match link.get_to() {
                    30 | 26 => {},
                    _ => assert!(false),
                },
                _ => assert!(false),
            }
        }
    })?;
    Ok(())
}
