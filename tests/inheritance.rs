mod setup;

use std::io;
use setup::setup_cfg;
use ssa::core::{ Edge };

#[test]
fn inheritance() -> io::Result<()> {
    setup_cfg("inheritance.sol", 23, |cfg| {
        let vertices = cfg.get_vertices();
        let edges = cfg.get_edges();
        let _stop = cfg.get_stop();
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
