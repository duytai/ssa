extern crate cfg;
mod setup;

use std::io;
use setup::setup;
use cfg::{ State, Shape };

#[test]
fn if_body_is_expression() -> io::Result<()> {
    setup("if_1.sol", |mut control_flow| {
        let State { vertices, edges, stop, .. } = control_flow.start_at(15).unwrap();
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
        let State { vertices, edges, stop, .. } = control_flow.start_at(16).unwrap();
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
        let State { vertices, edges, stop, .. } = control_flow.start_at(20).unwrap();
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
        let State { vertices, edges, stop, .. } = control_flow.start_at(21).unwrap();
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
        let State { vertices, edges, stop, .. } = control_flow.start_at(19).unwrap();
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
        let State { vertices, edges, stop, .. } = control_flow.start_at(37).unwrap();
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
        let State { vertices, edges, stop, .. } = control_flow.start_at(39).unwrap();
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
