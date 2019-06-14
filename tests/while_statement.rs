extern crate sg;
mod setup;

use std::io;
use setup::setup;
use sg::{ State, Shape, Vertex };

#[test]
fn while_body_is_expression() -> io::Result<()> {
    setup("while_1.sol", |mut control_flow| {
        let State { vertices, edges, stop, .. } = control_flow.analyze(15, vec![]).unwrap();
        assert_eq!(vertices.len(), 6);
        assert_eq!(edges.len(), 6);
        assert!(edges.contains(&(8, 12)));
        assert!(edges.contains(&(12, 8)));
        assert!(edges.contains(&(8, stop)));
    })?;
    Ok(())
}

#[test]
fn while_body_is_block() -> io::Result<()> {
    setup("while_2.sol", |mut control_flow| {
        let State { vertices, edges, stop, .. } = control_flow.analyze(16, vec![]).unwrap();
        assert_eq!(vertices.len(), 6);
        assert_eq!(edges.len(), 6);
        assert!(edges.contains(&(8, 12)));
        assert!(edges.contains(&(12, 8)));
        assert!(edges.contains(&(8, stop)));
    })?;
    Ok(())
}

#[test]
fn break_in_body() -> io::Result<()> {
    setup("while_3.sol", |mut control_flow| {
        let State { vertices, edges, stop, .. } = control_flow.analyze(17, vec![]).unwrap();
        assert_eq!(vertices.len(), 6);
        assert_eq!(edges.len(), 6);
        assert!(edges.contains(&(8, 9)));
        assert!(edges.contains(&(8, 9)));
        assert!(edges.contains(&(9, stop)));
    })?;
    Ok(())
}

#[test]
fn continue_in_body() -> io::Result<()> {
    setup("while_4.sol", |mut control_flow| {
        let State { vertices, edges, stop, .. } = control_flow.analyze(21, vec![]).unwrap();
        assert_eq!(vertices.len(), 7);
        assert_eq!(edges.len(), 7);
        assert!(edges.contains(&(8, 12)));
        assert!(edges.contains(&(12, 13)));
        assert!(edges.contains(&(13, 8)));
        assert!(edges.contains(&(8, stop)));
    })?;
    Ok(())
}
