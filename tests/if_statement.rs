extern crate sg;
mod setup;

use std::io;
use setup::setup;
use sg::{ State };

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
