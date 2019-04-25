use super::{
    walker::{ Walker },
};

#[derive(Debug)]
pub struct Graph<'a> {
    config: &'a GraphConfig<'a>,
    walker: Walker<'a>,
    root: GraphNode<'a>,
}

#[derive(Debug)]
pub struct GraphConfig<'a> {
    pub contract_name: &'a str,
    pub kind: GraphKind<'a>,
    pub include_state: bool,
}

#[derive(Debug)]
pub enum GraphKind<'a> {
    Constructor,
    Fallback,
    Function(&'a str),
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
    pub fn new(config: &'a GraphConfig, walker: Walker<'a>) -> Self {
        Graph { config, walker, root: GraphNode::None }
    }

    pub fn build_items(&mut self, walker: Walker<'a>) -> Vec<CodeBlock<'a>> {
        match walker.node.name {
            "IfStatement" => {
                let node = self.build_node(NodeKind::IfStatement, walker); 
                vec![CodeBlock::Link(Box::new(node))]
            },
            "WhileStatement" => {
                let node = self.build_node(NodeKind::WhileStatement, walker);
                vec![CodeBlock::Link(Box::new(node))]
            },
            "ForStatement" => {
                let node = self.build_node(NodeKind::ForStatement, walker);
                vec![CodeBlock::Link(Box::new(node))]
            },
            "DoWhileStatement" => {
                let node = self.build_node(NodeKind::DoWhileStatement, walker);
                vec![CodeBlock::Link(Box::new(node))]
            },
            "Return" => {
                let node = GraphNode::Return(CodeBlock::Block(walker));
                vec![CodeBlock::Link(Box::new(node))]
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
                walker.all(|walker| {
                    walker.node.name == "FunctionCall"
                }, |walkers| {
                    for walker in walkers {
                        let block = CodeBlock::Block(walker);
                        let node = GraphNode::FunctionCall(block);
                        blocks.push(CodeBlock::Link(Box::new(node)));
                    }
                });
                blocks.push(CodeBlock::Block(walker));
                blocks
            },
            "ExpressionStatement" => {
                let mut blocks = vec![];
                walker.all(|walker| {
                    walker.node.name == "FunctionCall"
                }, |walkers| {
                    for walker in walkers {
                        let mut funcs = (false, false, false, false, false);
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
                            ("suicide", "function (address)") => funcs.3 = true,
                            ("selfdestruct", "function (address)") => funcs.4 = true,
                            _ => {},
                        };
                        let block = CodeBlock::Block(walker);
                        match funcs {
                            (true, _, _, _, _) => {
                                blocks.push(CodeBlock::Link(Box::new(GraphNode::Revert(block))));
                            },
                            (_, true, _, _, _) => {
                                blocks.push(CodeBlock::Link(Box::new(GraphNode::Assert(block))));
                            },
                            (_, _, true, _, _) => {
                                blocks.push(CodeBlock::Link(Box::new(GraphNode::Require(block))));
                            },
                            (_, _, _, true, _) => {
                                blocks.push(CodeBlock::Link(Box::new(GraphNode::Suicide(block))));
                            },
                            (_, _, _, _, true) => {
                                blocks.push(CodeBlock::Link(Box::new(GraphNode::Selfdestruct(block))));
                            },
                            (_, _, _, _, _) => {
                                let node = GraphNode::FunctionCall(block);
                                blocks.push(CodeBlock::Link(Box::new(node)));
                            },
                        };
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
                            unimplemented!();
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
                let mut blocks = vec![]; 
                let contract_name = self.config.contract_name;
                let config_kind = &self.config.kind;
                walker.for_all(|walker| {
                    walker.node.name == "ContractDefinition"
                    && walker.node.attributes["name"].as_str().unwrap_or("") == contract_name
                }, |walkers| {
                    for walker in walkers {
                        if self.config.include_state {
                            walker.for_all(|walker| {
                                walker.node.name != "FunctionDefinition"
                            }, |walkers| {
                                for walker in walkers {
                                    let block = CodeBlock::Block(walker);
                                    blocks.push(block);
                                }
                            });
                        }
                        match config_kind {
                            GraphKind::Constructor => {
                                walker.for_all(|walker| {
                                    walker.node.attributes["isConstructor"].as_bool().unwrap_or(false)
                                }, |walkers| {
                                    for walker in walkers {
                                        blocks.append(
                                            &mut self.build_block(BlockKind::Param, walker)
                                        );
                                    }
                                });
                            },
                            GraphKind::Fallback => {
                                walker.for_all(|walker| {
                                    !walker.node.attributes["isConstructor"].as_bool().unwrap_or(false)
                                    && walker.node.attributes["name"].as_str().unwrap_or("") == ""
                                }, |walkers| {
                                    for walker in walkers {
                                        blocks.append(
                                            &mut self.build_block(BlockKind::Param, walker)
                                        );
                                    }
                                });
                            },
                            GraphKind::Function(name) => {
                                walker.for_all(|walker| {
                                    walker.node.attributes["name"].as_str().unwrap_or("") == *name
                                }, |walkers| {
                                    for walker in walkers {
                                        blocks.append(
                                            &mut self.build_block(BlockKind::Param, walker)
                                        );
                                    }
                                });
                            },
                        }
                    }
                });
                GraphNode::Root(blocks)
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
                walker.for_each(|walker, _| {
                    match walker.node.name {
                        "BinaryOperation" => {
                            condition = CodeBlock::Block(walker);
                        },
                        "Block" => {
                            blocks = self.build_block(BlockKind::Body, walker);
                        },
                        _ => {
                            blocks.append(&mut self.build_items(walker));
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
                            condition = CodeBlock::Block(walker);
                        },
                        "Block" => {
                            blocks = self.build_block(BlockKind::Body, walker);
                        },
                        _ => {
                            blocks.append(&mut self.build_items(walker));
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
                            condition = CodeBlock::Block(walker);
                        },
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
