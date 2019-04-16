use super::walker::{ Walker };

pub struct GraphNode<'a> {
    source: &'a str,
} 

pub struct Graph<'a> {
    walker: &'a Walker<'a>,
    source: &'a str,
}

impl<'a> Graph<'a> {
    pub fn new(walker: &'a Walker, source: &'a str) -> Self {
        Graph { walker, source }
    }

    pub fn build(&mut self) {
        self.walker.for_each(|contract| {
            let mut state_nodes: Vec<GraphNode> = vec![];
            for child in &contract.node.children {
                let node = Walker::parse(child);
                if node.name != "FunctionDefinition" {
                    let from = node.source_offset as usize;
                    let to = (node.source_offset + node.source_len) as usize;
                    let graph_node = GraphNode {
                        source: &self.source[from..=to],
                    };
                    state_nodes.push(graph_node);
                }
            }
            Graph::format(&state_nodes);
        });
    }

    pub fn format(nodes: &Vec<GraphNode>) {
        for node in nodes {
            println!("{}", node.source);
            println!("========");
        }
    }
}
