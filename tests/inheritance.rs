mod setup;

use std::io;
use setup::setup_cfg;
use ssa::core::{ State, Edge };

#[test]
fn complex_block() -> io::Result<()> {
    setup_cfg("inheritance.sol", 23, |State { vertices, edges, .. }| {
        assert_eq!(vertices.len(), 9);
        assert_eq!(edges.len(), 8);
        assert!(edges.contains(&Edge::new(3, 5)));
        assert!(edges.contains(&Edge::new(5, 8)));
        assert!(edges.contains(&Edge::new(8, 10)));
        assert!(edges.contains(&Edge::new(10, 17)));
        assert!(edges.contains(&Edge::new(17, 19)));
        assert!(edges.contains(&Edge::new(19, 20)));
    })?;
    Ok(())
}
