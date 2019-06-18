mod setup;

use std::io;
use setup::setup_cfg;
use ssa::core::{ State, Shape, Vertex, Edge };

#[test]
fn while_body_is_expression() -> io::Result<()> {
    setup_cfg("while_1.sol", 15, |State { vertices, edges, stop, .. }| {
        assert_eq!(vertices.len(), 6);
        assert_eq!(edges.len(), 6);
        assert!(edges.contains(&Edge::new(8, 12)));
        assert!(edges.contains(&Edge::new(12, 8)));
        assert!(edges.contains(&Edge::new(8, stop)));
    })?;
    Ok(())
}

#[test]
fn while_body_is_block() -> io::Result<()> {
    setup_cfg("while_2.sol", 16, |State { vertices, edges, stop, .. }| {
        assert_eq!(vertices.len(), 6);
        assert_eq!(edges.len(), 6);
        assert!(edges.contains(&Edge::new(8, 12)));
        assert!(edges.contains(&Edge::new(12, 8)));
        assert!(edges.contains(&Edge::new(8, stop)));
    })?;
    Ok(())
}

#[test]
fn break_in_body() -> io::Result<()> {
    setup_cfg("while_3.sol", 17, |State { vertices, edges, stop, .. }| {
        assert_eq!(vertices.len(), 6);
        assert_eq!(edges.len(), 6);
        assert!(edges.contains(&Edge::new(8, 9)));
        assert!(edges.contains(&Edge::new(8, 9)));
        assert!(edges.contains(&Edge::new(9, stop)));
    })?;
    Ok(())
}

#[test]
fn continue_in_body() -> io::Result<()> {
    setup_cfg("while_4.sol", 21, |State { vertices, edges, stop, .. }| {
        assert_eq!(vertices.len(), 7);
        assert_eq!(edges.len(), 7);
        assert!(edges.contains(&Edge::new(8, 12)));
        assert!(edges.contains(&Edge::new(12, 13)));
        assert!(edges.contains(&Edge::new(13, 8)));
        assert!(edges.contains(&Edge::new(8, stop)));
    })?;
    Ok(())
}
