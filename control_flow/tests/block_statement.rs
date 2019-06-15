extern crate control_flow;
mod setup;

use std::io;
use setup::setup;
use control_flow::{ State, Shape, Vertex };

#[test]
fn complex_block() -> io::Result<()> {
    setup("block_1.sol", |mut control_flow| {
        let State { vertices, edges, stop, .. } = control_flow.start_at(55).unwrap();
        assert_eq!(vertices.len(), 10);
        assert_eq!(edges.len(), 9);
        let function_calls = vertices.iter()
            .filter(|v| v.shape == Shape::DoubleCircle)
            .collect::<Vec<&Vertex>>();
        assert_eq!(function_calls.len(), 4);
    })?;
    Ok(())
}
