extern crate sg;
mod setup;

use std::io;
use setup::setup;
use sg::{ State, Shape };

#[test]
fn if_body_is_expression() -> io::Result<()> {
    setup("if_1.sol", |mut control_flow| {
        let State { vertices, edges, stop, .. } = control_flow.analyze(15, vec![]).unwrap();
        assert_eq!(vertices.len(), 6);
        assert_eq!(edges.len(), 6);
        assert!(edges.contains(&(8, 12)));
        assert!(edges.contains(&(8, stop)));
    })?;
    Ok(())
}

#[test]
fn if_body_is_block() -> io::Result<()> {
    setup("if_2.sol", |mut control_flow| {
        let State { vertices, edges, stop, .. } = control_flow.analyze(16, vec![]).unwrap();
        assert_eq!(vertices.len(), 6);
        assert_eq!(edges.len(), 6);
        assert!(edges.contains(&(8, 12)));
        assert!(edges.contains(&(8, stop)));
    })?;
    Ok(())
}

#[test]
fn else_body_is_expression() -> io::Result<()> {
    setup("if_3.sol", |mut control_flow| {
        let State { vertices, edges, stop, .. } = control_flow.analyze(20, vec![]).unwrap();
        assert_eq!(vertices.len(), 7);
        assert_eq!(edges.len(), 7);
        assert!(edges.contains(&(8, 12)));
        assert!(edges.contains(&(8, 17)));
        assert!(edges.contains(&(17, stop)));
        assert!(edges.contains(&(12, stop)));
    })?;
    Ok(())
}

#[test]
fn else_body_is_block() -> io::Result<()> {
    setup("if_4.sol", |mut control_flow| {
        let State { vertices, edges, stop, .. } = control_flow.analyze(21, vec![]).unwrap();
        assert_eq!(vertices.len(), 7);
        assert_eq!(edges.len(), 7);
        assert!(edges.contains(&(8, 12)));
        assert!(edges.contains(&(8, 17)));
        assert!(edges.contains(&(17, stop)));
        assert!(edges.contains(&(12, stop)));
    })?;
    Ok(())
}

#[test]
fn both_if_else_body_are_expression() -> io::Result<()> {
    setup("if_5.sol", |mut control_flow| {
        let State { vertices, edges, stop, .. } = control_flow.analyze(19, vec![]).unwrap();
        assert_eq!(vertices.len(), 7);
        assert_eq!(edges.len(), 7);
        assert!(edges.contains(&(8, 12)));
        assert!(edges.contains(&(8, 16)));
        assert!(edges.contains(&(16, stop)));
        assert!(edges.contains(&(12, stop)));
    })?;
    Ok(())
}

#[test]
fn condition_is_function() -> io::Result<()> {
    setup("if_6.sol", |mut control_flow| {
        let State { vertices, edges, stop, .. } = control_flow.analyze(37, vec![]).unwrap();
        let condition_vertex = vertices.iter().find(|x| x.id == 26).unwrap();
        assert_eq!(condition_vertex.shape, Shape::Mdiamond);
        assert_eq!(vertices.len(), 7);
        assert_eq!(edges.len(), 7);
        assert!(edges.contains(&(26, 34)));
        assert!(edges.contains(&(26, 30)));
        assert!(edges.contains(&(34, stop)));
        assert!(edges.contains(&(30, stop)));
    })?;
    Ok(())
}

#[test]
fn condition_is_function_and_expression() -> io::Result<()> {
    setup("if_7.sol", |mut control_flow| {
        let State { vertices, edges, stop, .. } = control_flow.analyze(39, vec![]).unwrap();
        let condition_vertex = vertices.iter().find(|x| x.id == 28).unwrap();
        assert_eq!(condition_vertex.shape, Shape::Diamond);
        assert_eq!(vertices.len(), 8);
        assert_eq!(edges.len(), 8);
        assert!(edges.contains(&(26, 28)));
        assert!(edges.contains(&(28, 36)));
        assert!(edges.contains(&(28, 32)));
        assert!(edges.contains(&(32, stop)));
        assert!(edges.contains(&(36, stop)));
    })?;
    Ok(())
}
