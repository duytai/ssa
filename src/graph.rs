use super::{
    walker::{ Walker },
};

#[derive(Debug)]
pub struct Graph<'a> {
    walker: Walker<'a>,
    root: GraphNode<'a>,
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

#[derive(Debug)]
pub enum CodeBlock<'a> {
    Block(Walker<'a>),
    Link(Box<GraphNode<'a>>),
    None,
}

#[derive(Debug)]
pub enum GraphNode<'a> {
    Root(Vec<CodeBlock<'a>>),
    IfStatement(IfStatement<'a>),
    WhileStatement(WhileStatement<'a>),
    ForStatement(ForStatement<'a>),
    DoWhileStatement(DoWhileStatement<'a>),
    Return(CodeBlock<'a>),
    Require(CodeBlock<'a>),
    Assert(CodeBlock<'a>),
    Revert(CodeBlock<'a>),
    Throw(CodeBlock<'a>),
    Break(CodeBlock<'a>),
    Continue(CodeBlock<'a>),
    Suicide(CodeBlock<'a>),
    Selfdestruct(CodeBlock<'a>),
    FunctionCall(CodeBlock<'a>),
    ModifierInvocation(CodeBlock<'a>),
    None,
}

#[derive(Debug)]
pub struct WhileStatement<'a> {
    pub condition: CodeBlock<'a>,
    pub blocks: Vec<CodeBlock<'a>>,
}

#[derive(Debug)]
pub struct DoWhileStatement<'a> {
    pub condition: CodeBlock<'a>,
    pub blocks: Vec<CodeBlock<'a>>,
}

#[derive(Debug)]
pub struct IfStatement<'a> {
    pub condition: CodeBlock<'a>,
    pub tblocks: Vec<CodeBlock<'a>>,
    pub fblocks: Vec<CodeBlock<'a>>,
}

#[derive(Debug)]
pub struct ForStatement<'a> {
    pub condition: CodeBlock<'a>,
    pub init: CodeBlock<'a>,
    pub expression: CodeBlock<'a>,  
    pub blocks: Vec<CodeBlock<'a>>,
}

impl<'a> Graph<'a> {
    pub fn new(walker: Walker<'a>) -> Self {
        Graph { walker, root: GraphNode::None }
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
                let node = GraphNode::Return(CodeBlock::Block(walker));
                blocks.push(CodeBlock::Link(Box::new(node)));
                blocks
            },
            "Throw" => {
                let node = GraphNode::Throw(CodeBlock::Block(walker));
                vec![CodeBlock::Link(Box::new(node))]
            },
            "Continue" => {
                let node = GraphNode::Continue(CodeBlock::Block(walker));
                vec![CodeBlock::Link(Box::new(node))]
            },
            "Break" => {
                let node = GraphNode::Break(CodeBlock::Block(walker));
                vec![CodeBlock::Link(Box::new(node))]
            },
            "VariableDeclarationStatement" | "EmitStatement" => {
                let mut blocks = vec![];
                blocks.push(CodeBlock::Block(walker));
                blocks
            },
            "ExpressionStatement" => {
                let mut blocks = vec![];
                walker.all(|walker| {
                    walker.node.name == "FunctionCall"
                }, |walkers| {
                    for walker in walkers {
                        walker.for_each(|w, index| {
                            if index == 0 {
                                let block = CodeBlock::Block(walker.clone());
                                let node_value = w.node.attributes["value"]
                                    .as_str()
                                    .unwrap_or("");
                                match node_value {
                                    "revert" => {
                                        blocks.push(CodeBlock::Link(Box::new(GraphNode::Revert(block))));
                                    },
                                    "assert" => {
                                        blocks.push(CodeBlock::Link(Box::new(GraphNode::Assert(block))));
                                    },
                                    "require" => {
                                        blocks.push(CodeBlock::Link(Box::new(GraphNode::Require(block))));
                                    },
                                    "suicide" => {
                                        blocks.push(CodeBlock::Link(Box::new(GraphNode::Suicide(block))));
                                    },
                                    "selfdestruct" => {
                                        blocks.push(CodeBlock::Link(Box::new(GraphNode::Selfdestruct(block))));
                                    },
                                    _ => {
                                        let node = GraphNode::FunctionCall(block);
                                        blocks.push(CodeBlock::Link(Box::new(node)));
                                    },
                                };
                            }
                        });
                    }
                });
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
                            let node = GraphNode::ModifierInvocation(block);
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

    pub fn build_node(&mut self, kind: NodeKind, walker: Walker<'a>) -> GraphNode<'a> {
        match kind {
            NodeKind::Root => {
                match walker.node.name {
                    "FunctionDefinition" | "ModifierDefinition" => {
                        GraphNode::Root(self.build_block(BlockKind::Param, walker))
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
                GraphNode::ForStatement(ForStatement { condition, init, expression, blocks })
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
                GraphNode::DoWhileStatement(DoWhileStatement { condition, blocks })
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
                GraphNode::WhileStatement(WhileStatement { condition, blocks })
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
                GraphNode::IfStatement(IfStatement { condition, tblocks, fblocks })
            },
        } 
    }

    pub fn update(&mut self) -> &GraphNode {
        match self.root {
            GraphNode::None => {
                self.root = self.build_node(NodeKind::Root, self.walker.clone());
                &self.root
            },
            _ => &self.root,
        }
    }
}
