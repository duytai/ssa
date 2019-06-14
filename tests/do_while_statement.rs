extern crate slint;
mod setup;

use std::io;
use setup::setup;
use slint::{ State, Shape, Vertex };

#[test]
fn do_body_is_expression() -> io::Result<()> {
    setup("do_while_1.sol", |mut control_flow| {
        let State { vertices, edges, stop, .. } = control_flow.analyze(15, vec![]).unwrap();
        assert_eq!(vertices.len(), 6);
        assert_eq!(edges.len(), 6);
        assert!(edges.contains(&(11, 12)));
        assert!(edges.contains(&(12, 11)));
        assert!(edges.contains(&(12, stop)));
    })?;
    Ok(())
}

#[test]
fn do_body_is_block() -> io::Result<()> {
    setup("do_while_2.sol", |mut control_flow| {
        let State { vertices, edges, stop, .. } = control_flow.analyze(16, vec![]).unwrap();
        assert_eq!(vertices.len(), 6);
        assert_eq!(edges.len(), 6);
        assert!(edges.contains(&(11, 13)));
        assert!(edges.contains(&(13, 11)));
        assert!(edges.contains(&(13, stop)));
    })?;
    Ok(())
}

#[test]
fn break_in_do() -> io::Result<()> {
    setup("do_while_3.sol", |mut control_flow| {
        let State { vertices, edges, stop, .. } = control_flow.analyze(17, vec![]).unwrap();
        assert_eq!(vertices.len(), 5);
        assert_eq!(edges.len(), 4);
        assert!(edges.contains(&(8, stop)));
    })?;
    Ok(())
}

#[test]
fn continue_in_do() -> io::Result<()> {
    setup("do_while_4.sol", |mut control_flow| {
        let State { vertices, edges, stop, .. } = control_flow.analyze(17, vec![]).unwrap();
        assert_eq!(vertices.len(), 6);
        assert_eq!(edges.len(), 6);
        assert!(edges.contains(&(8, 14)));
        assert!(edges.contains(&(14, 8)));
        assert!(edges.contains(&(14, stop)));
    })?;
    Ok(())
}
