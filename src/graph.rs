use std::rc::Rc;
use super::walker::{ Walker };

enum GraphLink {
    None,
    More(Rc<GraphNode>),
}

pub struct GraphNode {
    source: String,
    link_true: GraphLink,
    link_false: GraphLink,
} 

pub struct Graph<'a> {
    walker: &'a Walker<'a>,
    source: &'a str,
}

impl<'a> Graph<'a> {
    pub fn new(walker: &'a Walker, source: &'a str) -> Self {
        Graph { walker, source }
    }

    pub fn build(&self) {
        self.walker.for_each(|contract| {
            for child in &contract.node.children {
                let node = Walker::parse(child);
                match node.name {
                    "FunctionDefinition" => {
                    },
                    _ => {
                    },
                }
            }
        });
    }
}
