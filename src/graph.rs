use super::walker::{ Walker };

#[derive(Debug)]
pub struct Graph<'a> {
    walker: &'a Walker<'a>,
    source: &'a str,
    root: GraphNode,
}

#[derive(Debug)]
pub enum BlockKind {
    Constructor,
    BlockBody,
}

#[derive(Debug)]
pub enum NodeKind {
    Root,
    IfStatement,
    WhileStatement,
    ForStatement,
    DoWhileStatement,
}

#[derive(Debug)]
pub struct BlockContent {
    source: String,
    id: u32,
}

#[derive(Debug)]
pub enum CodeBlock {
    Block(BlockContent),
    Link(Box<GraphNode>),
    None,
}

#[derive(Debug)]
pub enum GraphNode {
    Root(Vec<CodeBlock>),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    ForStatement(ForStatement),
    DoWhileStatement(DoWhileStatement),
    Return(CodeBlock),
    Require(CodeBlock),
    Assert(CodeBlock),
    Revert,
    Throw,
    Break,
    Continue,
    None,
}

#[derive(Debug)]
pub struct WhileStatement {
    condition: CodeBlock,
    blocks: Vec<CodeBlock>,
}

#[derive(Debug)]
pub struct DoWhileStatement {
    condition: CodeBlock,
    blocks: Vec<CodeBlock>,
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: CodeBlock,
    pub tblocks: Vec<CodeBlock>,
    pub fblocks: Vec<CodeBlock>,
}

#[derive(Debug)]
pub struct ForStatement {
    steps: Vec<CodeBlock>,
    blocks: Vec<CodeBlock>,
}

impl<'a> Graph<'a> {
    pub fn new(walker: &'a Walker, source: &'a str) -> Self {
        Graph { walker, source, root: GraphNode::None }
    }

    pub fn build_item(&mut self, walker: &Walker) -> CodeBlock {

        let from = walker.node.source_offset as usize;
        let to = from + walker.node.source_len as usize;
        let source = &self.source[from..to];
        let block = CodeBlock::Block(BlockContent {
            source: source.to_string(),
            id: walker.node.id,
        });

        match walker.node.name {
            "IfStatement" => {
                let node = self.build_node(NodeKind::IfStatement, walker); 
                CodeBlock::Link(Box::new(node))
            },
            "WhileStatement" => {
                let node = self.build_node(NodeKind::WhileStatement, walker);
                CodeBlock::Link(Box::new(node))
            },
            "ForStatement" => {
                let node = self.build_node(NodeKind::ForStatement, walker);
                CodeBlock::Link(Box::new(node))
            },
            "DoWhileStatement" => {
                let node = self.build_node(NodeKind::DoWhileStatement, walker);
                CodeBlock::Link(Box::new(node))
            },
            "Return" => {
                let node = GraphNode::Return(block);
                CodeBlock::Link(Box::new(node))
            },
            "Throw" => {
                CodeBlock::Link(Box::new(GraphNode::Throw))
            },
            "Continue" => {
                CodeBlock::Link(Box::new(GraphNode::Continue))
            },
            "Break" => {
                CodeBlock::Link(Box::new(GraphNode::Break))
            },
            _ => {
                match walker.node.name {
                    "ExpressionStatement" => {
                        let mut funcs = (false, false, false);
                        walker.for_each(|walker, _| {
                            if walker.node.name == "FunctionCall" {
                                walker.for_each(|walker, _| {
                                    if walker.node.name == "Identifier" {
                                        let node_value = walker.node.attributes["value"]
                                            .as_str()
                                            .unwrap_or("");
                                        let node_type = walker.node.attributes["type"] 
                                            .as_str()
                                            .unwrap_or("");
                                        match(node_value, node_type) {
                                            ("revert", "function () pure") => funcs.0 = true,
                                            ("assert", "function (bool) pure") => funcs.1 = true,
                                            ("require", "function (bool) pure") =>  funcs.2 = true,
                                            _ => {},
                                        }
                                    }
                                })
                            }
                        });
                        match funcs {
                            (true, _, _) => {
                                CodeBlock::Link(Box::new(GraphNode::Revert))
                            },
                            (_, true, _) => {
                                CodeBlock::Link(Box::new(GraphNode::Assert(block)))
                            },
                            (_, _, true) => {
                                CodeBlock::Link(Box::new(GraphNode::Require(block)))
                            },
                            (_, _, _) => block,
                        }
                    },
                    _ => block,
                }
            },
        }
    }

    pub fn build_block(&mut self, kind: BlockKind, walker: &Walker) -> Vec<CodeBlock> {
        let mut blocks = vec![];
        match kind {
            BlockKind::BlockBody => {
                walker.for_each(|walker, _| {
                    let block = self.build_item(walker);
                    blocks.push(block);
                })
            },
            BlockKind::Constructor => {
                walker.for_each(|walker, index| {
                    if walker.node.name == "ParameterList" && index == 0 {
                        let from = walker.node.source_offset as usize;
                        let to = from + walker.node.source_len as usize;
                        let source = &self.source[from..to];
                        let block = CodeBlock::Block(BlockContent {
                            source: source.to_string(),
                            id: walker.node.id,
                        });
                        blocks.push(block);
                    }
                    if walker.node.name == "Block" {
                        blocks.append(&mut self.build_block(BlockKind::BlockBody, walker));
                    }
                })
            },
        }
        blocks
    } 

    pub fn build_node(&mut self, kind: NodeKind, walker: &Walker) -> GraphNode {
        match kind {
            NodeKind::Root => {
                let mut state_blocks = vec![];
                let mut constructor_blocks = vec![];
                let mut blocks = vec![]; 
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
                                            &mut self.build_block(BlockKind::Constructor, walker)
                                        );
                                    }
                                },
                                _ => {
                                    let from = walker.node.source_offset as usize;
                                    let to = from + walker.node.source_len as usize; 
                                    let source = &self.source[from..=to];
                                    let block = CodeBlock::Block(BlockContent {
                                        source: source.to_string(),
                                        id: walker.node.id,
                                    });
                                    state_blocks.push(block);
                                }
                            }
                        });
                    }
                });
                blocks.append(&mut state_blocks);
                blocks.append(&mut constructor_blocks);
                GraphNode::Root(blocks)
            },
            NodeKind::ForStatement => {
                let mut blocks = vec![];
                let mut steps = vec![];
                let walker_len = walker.len();
                walker.for_each(|walker, index| {
                    if index == walker_len - 1 {
                        if walker.node.name == "Block" {
                            blocks = self.build_block(BlockKind::BlockBody, walker);
                        } else {
                            blocks.push(self.build_item(walker));
                        }
                    } else {
                        let from = walker.node.source_offset as usize;
                        let to = from + walker.node.source_len as usize;
                        let source = &self.source[from..=to];
                        let block = CodeBlock::Block(BlockContent {
                            source: source.to_string(),
                            id: walker.node.id,
                        });
                        steps.push(block);
                    }
                });
                GraphNode::ForStatement(ForStatement { steps, blocks })
            },
            NodeKind::DoWhileStatement => {
                let mut condition = CodeBlock::None; 
                let mut blocks = vec![];
                walker.for_each(|walker, _| {
                    match walker.node.name {
                        "BinaryOperation" => {
                            let from = walker.node.source_offset as usize;
                            let to = from + walker.node.source_len as usize;
                            let source = &self.source[from..=to];
                            condition = CodeBlock::Block(BlockContent {
                                source: source.to_string(),
                                id: walker.node.id,
                            });
                        },
                        "Block" => {
                            blocks = self.build_block(BlockKind::BlockBody, walker);
                        },
                        _ => {
                            blocks.push(self.build_item(walker));
                        },
                    }
                });
                GraphNode::DoWhileStatement(DoWhileStatement { condition, blocks })
            },
            NodeKind::WhileStatement => {
                let mut condition = CodeBlock::None; 
                let mut blocks = vec![];
                walker.for_each(|walker, _| {
                    match walker.node.name {
                        "BinaryOperation" => {
                            let from = walker.node.source_offset as usize;
                            let to = from + walker.node.source_len as usize;
                            let source = &self.source[from..=to];
                            condition = CodeBlock::Block(BlockContent {
                                source: source.to_string(),
                                id: walker.node.id,
                            });
                        },
                        "Block" => {
                            blocks = self.build_block(BlockKind::BlockBody, walker);
                        },
                        _ => {
                            blocks.push(self.build_item(walker));
                        },
                    }
                });
                GraphNode::WhileStatement(WhileStatement { condition, blocks })
            },
            NodeKind::IfStatement => {
                let mut condition = CodeBlock::None; 
                let mut tblocks = vec![];
                let mut fblocks = vec![];
                walker.for_each(|walker, index | {
                    match walker.node.name {
                        "BinaryOperation" => {
                            let from = walker.node.source_offset as usize;
                            let to = from + walker.node.source_len as usize;
                            let source = &self.source[from..=to];
                            condition = CodeBlock::Block(BlockContent {
                                source: source.to_string(),
                                id: walker.node.id,
                            });
                        },
                        "Block" => {
                            if index == 1 {
                                tblocks = self.build_block(BlockKind::BlockBody, walker);
                            } else {
                                fblocks = self.build_block(BlockKind::BlockBody, walker);
                            }
                        },
                        _ => {
                            tblocks.push(self.build_item(walker));
                        }
                    }
                });
                GraphNode::IfStatement(IfStatement { condition, tblocks, fblocks })
            },
        } 
    }

    pub fn update(&mut self) -> &GraphNode {
        match self.root {
            GraphNode::None => {
                self.root = self.build_node(NodeKind::Root, self.walker);
                &self.root
            },
            _ => &self.root,
        }
    }
}
