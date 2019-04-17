use super::walker::{ Walker };

#[derive(Debug)]
pub struct Graph<'a> {
    walker: &'a Walker<'a>,
    source: &'a str,
    root: Option<GraphNode>,
}

#[derive(Debug)]
pub enum GraphNodeKind {
    Root,
    Constructor,
    FunctionBody,
}

#[derive(Debug)]
pub enum CodeBlock {
    Block(String),
    Link(Box<GraphNode>),
}

#[derive(Debug)]
pub struct GraphNode {
    kind: GraphNodeKind,
    blocks: Vec<CodeBlock>,
}

impl<'a> Graph<'a> {
    pub fn new(walker: &'a Walker, source: &'a str) -> Self {
        Graph { walker, source, root: None }
    }

    pub fn build_block(&mut self, kind: GraphNodeKind, walker: &Walker) -> Vec<CodeBlock> {
        let mut blocks = vec![];
        match kind {
            GraphNodeKind::FunctionBody => {
                walker.for_each(|walker, index| {
                    match walker.node.name {
                        "IfStatement" => {},
                        _ => {
                            let from = walker.node.source_offset as usize;
                            let to = from + walker.node.source_len as usize;
                            let source = &self.source[from..to];
                            let block = CodeBlock::Block(source.to_string());
                            blocks.push(block);
                        }
                    }
                })
            },
            GraphNodeKind::Constructor => {
                walker.for_each(|walker, index| {
                    if walker.node.name == "ParameterList" && index == 0 {
                        let from = walker.node.source_offset as usize;
                        let to = from + walker.node.source_len as usize;
                        let source = &self.source[from..to];
                        let block = CodeBlock::Block(source.to_string());
                        blocks.push(block);
                    }
                    if walker.node.name == "Block" && index == 2 {
                        blocks.append(&mut self.build_block(GraphNodeKind::FunctionBody, walker));
                    }
                })
            },
            _ => {},
        }
        blocks
    } 

    pub fn build_node(&mut self, kind: GraphNodeKind, walker: &Walker) -> GraphNode {
        let mut node = GraphNode { kind, blocks: vec![] };
        match node.kind {
            GraphNodeKind::Root => {
                let mut state_blocks = vec![];
                let mut constructor_blocks = vec![];
                walker.for_each(|walker, _| {
                    if walker.node.name == "ContractDefinition" {
                        walker.for_each(|walker, _| {
                            let is_constructor = walker.node
                                .attributes["isConstructor"]
                                .as_bool()
                                .unwrap_or(false);
                            match walker.node.name {
                                "FunctionDefinition" => {
                                    if is_constructor {
                                        constructor_blocks.append(
                                            &mut self.build_block(GraphNodeKind::Constructor, walker)
                                        );
                                    }
                                },
                                _ => {
                                    let from = walker.node.source_offset as usize;
                                    let to = from + walker.node.source_len as usize; 
                                    let source = &self.source[from..=to];
                                    let block = CodeBlock::Block(source.to_string());
                                    state_blocks.push(block);
                                }
                            }
                        });
                    }
                });
                node.blocks.append(&mut state_blocks);
                node.blocks.append(&mut constructor_blocks);
                println!("{:?}", node);
            },
            _ => {},
        } 
        node
    }

    pub fn build(&mut self) {
        self.root = Some(self.build_node(GraphNodeKind::Root, self.walker));
    }
}
