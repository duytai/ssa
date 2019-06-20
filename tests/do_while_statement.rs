mod setup;

use std::io;
use setup::setup_cfg;
use ssa::core::{ State, Edge };

#[test]
fn do_body_is_expression() -> io::Result<()> {
    setup_cfg("do_while_1.sol", 15, |State { vertices, edges, stop, ..}| {
        assert_eq!(vertices.len(), 6);
        assert_eq!(edges.len(), 6);
        assert!(edges.contains(&Edge::new(11, 12)));
        assert!(edges.contains(&Edge::new(12, 11)));
        assert!(edges.contains(&Edge::new(12, stop)));
    })?;
    Ok(())
}

#[test]
fn do_body_is_block() -> io::Result<()> {
    setup_cfg("do_while_2.sol", 16, |State {vertices, edges, stop, ..}| {
        assert_eq!(vertices.len(), 6);
        assert_eq!(edges.len(), 6);
        assert!(edges.contains(&Edge::new(11, 13)));
        assert!(edges.contains(&Edge::new(13, 11)));
        assert!(edges.contains(&Edge::new(13, stop)));
    })?;
    Ok(())
}

#[test]
fn break_in_do() -> io::Result<()> {
    setup_cfg("do_while_3.sol", 17, |State { vertices, edges, stop, ..}| {
        assert_eq!(vertices.len(), 5);
        assert_eq!(edges.len(), 4);
        assert!(edges.contains(&Edge::new(8, stop)));
    })?;
    Ok(())
}

#[test]
fn continue_in_do() -> io::Result<()> {
    setup_cfg("do_while_4.sol", 17, |State { vertices, edges, stop, ..}| {
        assert_eq!(vertices.len(), 6);
        assert_eq!(edges.len(), 6);
        assert!(edges.contains(&Edge::new(8, 14)));
        assert!(edges.contains(&Edge::new(14, 8)));
        assert!(edges.contains(&Edge::new(14, stop)));
    })?;
    Ok(())
}
