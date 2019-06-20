mod setup;

use std::io;
use setup::setup_cfg;
use ssa::core::{ State, Shape, Edge };

#[test]
fn if_body_is_expression() -> io::Result<()> {
    setup_cfg("if_1.sol", 15, |State { vertices, edges, stop, .. }| {
        assert_eq!(vertices.len(), 6);
        assert_eq!(edges.len(), 6);
        assert!(edges.contains(&Edge::new(8, 12)));
        assert!(edges.contains(&Edge::new(8, stop)));
    })?;
    Ok(())
}

#[test]
fn if_body_is_block() -> io::Result<()> {
    setup_cfg("if_2.sol", 16, |State { vertices, edges, stop, .. }| {
        assert_eq!(vertices.len(), 6);
        assert_eq!(edges.len(), 6);
        assert!(edges.contains(&Edge::new(8, 12)));
        assert!(edges.contains(&Edge::new(8, stop)));
    })?;
    Ok(())
}

#[test]
fn else_body_is_expression() -> io::Result<()> {
    setup_cfg("if_3.sol", 20, |State { vertices, edges, stop, .. }| {
        assert_eq!(vertices.len(), 7);
        assert_eq!(edges.len(), 7);
        assert!(edges.contains(&Edge::new(8, 12)));
        assert!(edges.contains(&Edge::new(8, 17)));
        assert!(edges.contains(&Edge::new(17, stop)));
        assert!(edges.contains(&Edge::new(12, stop)));
    })?;
    Ok(())
}

#[test]
fn else_body_is_block() -> io::Result<()> {
    setup_cfg("if_4.sol", 21, |State { vertices, edges, stop, ..}| {
        assert_eq!(vertices.len(), 7);
        assert_eq!(edges.len(), 7);
        assert!(edges.contains(&Edge::new(8, 12)));
        assert!(edges.contains(&Edge::new(8, 17)));
        assert!(edges.contains(&Edge::new(17, stop)));
        assert!(edges.contains(&Edge::new(12, stop)));
    })?;
    Ok(())
}

#[test]
fn both_if_else_body_are_expression() -> io::Result<()> {
    setup_cfg("if_5.sol", 19, |State { vertices, edges, stop, ..}| {
        assert_eq!(vertices.len(), 7);
        assert_eq!(edges.len(), 7);
        assert!(edges.contains(&Edge::new(8, 12)));
        assert!(edges.contains(&Edge::new(8, 16)));
        assert!(edges.contains(&Edge::new(16, stop)));
        assert!(edges.contains(&Edge::new(12, stop)));
    })?;
    Ok(())
}

#[test]
fn condition_is_function() -> io::Result<()> {
    setup_cfg("if_6.sol", 37, |State { vertices, edges, stop, .. }| {
        let condition_vertex = vertices.iter().find(|v| {
            v.get_id() == 26
        }).unwrap();
        assert_eq!(condition_vertex.get_shape(), &Shape::Mdiamond);
        assert_eq!(vertices.len(), 7);
        assert_eq!(edges.len(), 7);
        assert!(edges.contains(&Edge::new(26, 34)));
        assert!(edges.contains(&Edge::new(26, 30)));
        assert!(edges.contains(&Edge::new(34, stop)));
        assert!(edges.contains(&Edge::new(30, stop)));
    })?;
    Ok(())
}

#[test]
fn condition_is_function_and_expression() -> io::Result<()> {
    setup_cfg("if_7.sol", 39, |State { vertices, edges, stop, .. }| {
        let condition_vertex = vertices.iter().find(|v| {
            v.get_id() == 28 
        }).unwrap();
        assert_eq!(condition_vertex.get_shape(), &Shape::Diamond);
        assert_eq!(vertices.len(), 8);
        assert_eq!(edges.len(), 8);
        assert!(edges.contains(&Edge::new(26, 28)));
        assert!(edges.contains(&Edge::new(28, 36)));
        assert!(edges.contains(&Edge::new(28, 32)));
        assert!(edges.contains(&Edge::new(32, stop)));
        assert!(edges.contains(&Edge::new(36, stop)));
    })?;
    Ok(())
}
