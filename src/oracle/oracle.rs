use std::collections::HashSet;
use crate::{
    vertex::Vertex,
    dict::Dictionary
};

pub trait Oracle {
    fn analyze(
        &mut self,
        edges: &HashSet<(u32, u32)>,
        vertices: &HashSet<Vertex>,
        dict: &Dictionary
    );
}
