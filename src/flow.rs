use json;
use super::{
    graph::{ Graph },
    walker::{ Walker },
};

pub struct Flow<'a>(Graph<'a>);

impl<'a> Flow<'a> {
    pub fn new(value: &'a json::JsonValue, source: &'a str) -> Self {
        let walker = Walker::new(value);
        let mut graph = Graph::new(&walker, source);
        graph.update();
        Flow(graph)
    }
}
