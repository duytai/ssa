use crate::walker::Walker;
use crate::code_block::{
    CodeBlock,
    BlockNode,
    IfStatement,
    WhileStatement,
    DoWhileStatement,
    ForStatement,
};

#[derive(Debug)]
pub struct Graph<'a> {
    walker: Walker<'a>,
    root: BlockNode<'a>,
}

#[derive(Debug)]
pub enum BlockKind {
    Param,
    Body,
}

#[derive(Debug)]
pub enum NodeKind {
    Root,
    IfStatement,
    WhileStatement,
    ForStatement,
    DoWhileStatement,
}

impl<'a> Graph<'a> {
    pub fn new(walker: Walker<'a>) -> Self {
        Graph { walker, root: BlockNode::None }
    }

    fn to_call(&self, walker: Walker<'a>) -> Option<CodeBlock<'a>> {
        let mut function_name = None;
        if walker.node.name == "FunctionCall" {
            walker.for_each(|walker, index| {
                if index == 0 {
                    let temp = walker.node.attributes["value"].as_str().unwrap_or("");
                    function_name = Some(temp);
                }
            });
        }
        let block = CodeBlock::Block(walker);
        function_name.map(|function_name| {
            match function_name {
                "revert" => CodeBlock::Link(Box::new(BlockNode::Revert(block))),
                "assert" => CodeBlock::Link(Box::new(BlockNode::Assert(block))),
                "require" => CodeBlock::Link(Box::new(BlockNode::Require(block))),
                "suicide" => CodeBlock::Link(Box::new(BlockNode::Suicide(block))),
                "selfdestruct" => CodeBlock::Link(Box::new(BlockNode::Selfdestruct(block))),
                _ => CodeBlock::Link(Box::new(BlockNode::FunctionCall(block))),
            }
        })
    }

    pub fn build_items(&mut self, walker: Walker<'a>) -> Vec<CodeBlock<'a>> {
        match walker.node.name {
            "IfStatement" => {
                let mut blocks = vec![];
                let node = self.build_node(NodeKind::IfStatement, walker); 
                blocks.push(CodeBlock::Link(Box::new(node)));
                blocks
            },
            "WhileStatement" => {
                let mut blocks = vec![];
                let node = self.build_node(NodeKind::WhileStatement, walker);
                blocks.push(CodeBlock::Link(Box::new(node)));
                blocks
            },
            "ForStatement" => {
                let mut blocks = vec![];
                let node = self.build_node(NodeKind::ForStatement, walker);
                blocks.push(CodeBlock::Link(Box::new(node)));
                blocks
            },
            "DoWhileStatement" => {
                let mut blocks = vec![];
                let node = self.build_node(NodeKind::DoWhileStatement, walker);
                blocks.push(CodeBlock::Link(Box::new(node)));
                blocks
            },
            "Return" => {
                let mut blocks = vec![];
                let node = BlockNode::Return(CodeBlock::Block(walker));
                blocks.push(CodeBlock::Link(Box::new(node)));
                blocks
            },
            "Throw" => {
                let node = BlockNode::Throw(CodeBlock::Block(walker));
                vec![CodeBlock::Link(Box::new(node))]
            },
            "Continue" => {
                let node = BlockNode::Continue(CodeBlock::Block(walker));
                vec![CodeBlock::Link(Box::new(node))]
            },
            "Break" => {
                let node = BlockNode::Break(CodeBlock::Block(walker));
                vec![CodeBlock::Link(Box::new(node))]
            },
            "VariableDeclarationStatement" | "EmitStatement" | "ExpressionStatement" => {
                let mut blocks = vec![];
                blocks.push(CodeBlock::Block(walker));
                blocks
            },
            "InlineAssemblyStatement" => unimplemented!(),
            "PlaceholderStatement" => unimplemented!(), 
            _ => vec![CodeBlock::Block(walker)],
        }
    }

    pub fn build_block(&mut self, kind: BlockKind, walker: Walker<'a>) -> Vec<CodeBlock<'a>> {
        let mut blocks = vec![];
        match kind {
            BlockKind::Body => {
                walker.for_each(|walker, _| {
                    blocks.append(&mut self.build_items(walker));
                })
            },
            BlockKind::Param => {
                walker.for_each(|walker, index| {
                    match walker.node.name {
                        "ParameterList" => {
                            if index == 0 {
                                let block = CodeBlock::Block(walker);
                                blocks.push(block);
                            }
                        },
                        "ModifierInvocation" => {
                            let block = CodeBlock::Block(walker);
                            let node = BlockNode::ModifierInvocation(block);
                            blocks.push(CodeBlock::Link(Box::new(node)));
                        },
                        "Block" => {
                            blocks.append(&mut self.build_block(BlockKind::Body, walker));
                        },
                        _ => {},
                    }
                })
            },
        }
        blocks
    } 

    pub fn build_node(&mut self, kind: NodeKind, walker: Walker<'a>) -> BlockNode<'a> {
        match kind {
            NodeKind::Root => {
                match walker.node.name {
                    "FunctionDefinition" | "ModifierDefinition" => {
                        BlockNode::Root(self.build_block(BlockKind::Param, walker))
                    },
                    _ => {
                        println!("name: {}", walker.node.name);
                        panic!("Entry point of graph must be a function");
                    }
                }
            },
            NodeKind::ForStatement => {
                let mut blocks = vec![];
                let mut condition = CodeBlock::None;
                let mut init = CodeBlock::None;
                let mut expression = CodeBlock::None;
                let mut props = vec!["initializationExpression", "condition", "loopExpression", "body"];
                for (key, _) in walker.node.attributes.entries() {
                    props = props.iter().filter_map(|x| {
                        if x == &key { return None; }
                        Some(*x)
                    }).collect();
                }
                walker.for_each(|walker, index| {
                    match props[index] {
                        "initializationExpression" => {
                            init = CodeBlock::Block(walker);
                        },
                        "condition" => {
                            condition = CodeBlock::Block(walker);
                        },
                        "loopExpression" => {
                            expression = CodeBlock::Block(walker);
                        },
                        _ => {
                            if walker.node.name == "Block" {
                                blocks = self.build_block(BlockKind::Body, walker);
                            } else {
                                blocks.append(&mut self.build_items(walker));
                            }
                        },
                    }
                });
                BlockNode::ForStatement(ForStatement { condition, init, expression, blocks })
            },
            NodeKind::DoWhileStatement => {
                let mut condition = CodeBlock::None; 
                let mut blocks = vec![];
                walker.for_all(|_| true, |walkers| {
                    for (index, walker) in walkers.into_iter().enumerate() {
                        if index == 0 {
                            condition = CodeBlock::Block(walker);
                        } else {
                            match walker.node.name {
                                "Block" => {
                                    blocks = self.build_block(BlockKind::Body, walker);
                                },
                                _ => {
                                    blocks.append(&mut self.build_items(walker));
                                }
                            }
                        }
                    }
                });
                BlockNode::DoWhileStatement(DoWhileStatement { condition, blocks })
            },
            NodeKind::WhileStatement => {
                let mut condition = CodeBlock::None; 
                let mut blocks = vec![];
                walker.for_all(|_| true, |walkers| {
                    for (index, walker) in walkers.into_iter().enumerate() {
                        if index == 0 {
                            condition = CodeBlock::Block(walker);
                        } else {
                            match walker.node.name {
                                "Block" => {
                                    blocks = self.build_block(BlockKind::Body, walker);
                                },
                                _ => {
                                    blocks.append(&mut self.build_items(walker));
                                }
                            }
                        }
                    }
                });
                BlockNode::WhileStatement(WhileStatement { condition, blocks })
            },
            NodeKind::IfStatement => {
                let mut condition = CodeBlock::None; 
                let mut tblocks = vec![];
                let mut fblocks = vec![];
                walker.for_all(|_| true, |walkers| {
                    for (index, walker) in walkers.into_iter().enumerate() {
                        if index == 0 {
                            condition = CodeBlock::Block(walker);
                        } else {
                            match walker.node.name {
                                "Block" => {
                                    if index == 1 {
                                        tblocks = self.build_block(BlockKind::Body, walker);
                                    } else {
                                        fblocks = self.build_block(BlockKind::Body, walker);
                                    }
                                },
                                _ => {
                                    tblocks.append(&mut self.build_items(walker));
                                }
                            }
                        }
                    }
                });
                BlockNode::IfStatement(IfStatement { condition, tblocks, fblocks })
            },
        } 
    }

    pub fn update(&mut self) -> &BlockNode {
        match self.root {
            BlockNode::None => {
                self.root = self.build_node(NodeKind::Root, self.walker.clone());
                &self.root
            },
            _ => &self.root,
        }
    }
}
