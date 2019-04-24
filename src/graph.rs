use super::{
    walker::{ Walker, BlockContent },
};

#[derive(Debug)]
pub struct Graph<'a> {
    config: &'a GraphConfig<'a>,
    walker: &'a Walker<'a>,
    source: &'a str,
    root: GraphNode,
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
    Revert(CodeBlock),
    Throw(CodeBlock),
    Break(CodeBlock),
    Continue(CodeBlock),
    Suicide(CodeBlock),
    Selfdestruct(CodeBlock),
    FunctionCall(CodeBlock),
    None,
}

#[derive(Debug)]
pub struct WhileStatement {
    pub condition: CodeBlock,
    pub blocks: Vec<CodeBlock>,
}

#[derive(Debug)]
pub struct DoWhileStatement {
    pub condition: CodeBlock,
    pub blocks: Vec<CodeBlock>,
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: CodeBlock,
    pub tblocks: Vec<CodeBlock>,
    pub fblocks: Vec<CodeBlock>,
}

#[derive(Debug)]
pub struct ForStatement {
    pub condition: CodeBlock,
    pub init: CodeBlock,
    pub expression: CodeBlock,  
    pub blocks: Vec<CodeBlock>,
}

impl<'a> Graph<'a> {
    pub fn new(config: &'a GraphConfig, walker: &'a Walker, source: &'a str) -> Self {
        Graph { config, walker, source, root: GraphNode::None }
    }

    pub fn build_items(&mut self, walker: &Walker) -> Vec<CodeBlock> {
        let block = CodeBlock::Block(walker.to_block_content(self.source));
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
                let node = GraphNode::Return(block);
                vec![CodeBlock::Link(Box::new(node))]
            },
            "Throw" => {
                let node = GraphNode::Throw(block);
                vec![CodeBlock::Link(Box::new(node))]
            },
            "Continue" => {
                let node = GraphNode::Continue(block);
                vec![CodeBlock::Link(Box::new(node))]
            },
            "Break" => {
                let node = GraphNode::Break(block);
                vec![CodeBlock::Link(Box::new(node))]
            },
            "VariableDeclarationStatement" | "EmitStatement" => {
                let mut blocks = vec![];
                walker.all(|walker| {
                    walker.node.name == "FunctionCall"
                }, |walkers| {
                    for walker in walkers.iter() {
                        let block = CodeBlock::Block(walker.to_block_content(self.source));
                        let node = GraphNode::FunctionCall(block);
                        blocks.push(CodeBlock::Link(Box::new(node)));
                    }
                });
                blocks.push(block);
                blocks
            },
            "ExpressionStatement" => {
                let mut blocks = vec![];
                walker.all(|walker| {
                    walker.node.name == "FunctionCall"
                }, |walkers| {
                    for walker in walkers.iter() {
                        let block = CodeBlock::Block(walker.to_block_content(self.source));
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
                blocks.push(block);
                blocks
            },
            "InlineAssemblyStatement" => unimplemented!(),
            "PlaceholderStatement" => unimplemented!(), 
            _ => vec![block],
        }
    }

    pub fn build_block(&mut self, kind: BlockKind, walker: &Walker) -> Vec<CodeBlock> {
        let mut blocks = vec![];
        match kind {
            BlockKind::Body => {
                walker.for_each(|walker, _| {
                    blocks.append(&mut self.build_items(walker));
                })
            },
            BlockKind::Param => {
                walker.for_each(|walker, index| {
                    if walker.node.name == "ParameterList" && index == 0 {
                        let block = CodeBlock::Block(walker.to_block_content(self.source));
                        blocks.push(block);
                    }
                    if walker.node.name == "ModifierInvocation" {
                        unimplemented!();
                    }
                    if walker.node.name == "Block" {
                        blocks.append(&mut self.build_block(BlockKind::Body, walker));
                    }
                })
            },
        }
        blocks
    } 

    pub fn build_node(&mut self, kind: NodeKind, walker: &Walker) -> GraphNode {
        match kind {
            NodeKind::Root => {
                let mut blocks = vec![]; 
                let contract_name = self.config.contract_name;
                let config_kind = &self.config.kind;
                walker.for_all(|walker| {
                    walker.node.name == "ContractDefinition"
                    && walker.node.attributes["name"].as_str().unwrap_or("") == contract_name
                }, |walkers| {
                    for walker in walkers.iter() {
                        if self.config.include_state {
                            walker.for_all(|walker| {
                                walker.node.name != "FunctionDefinition"
                            }, |walkers| {
                                for walker in walkers {
                                    let block = CodeBlock::Block(walker.to_block_content(self.source));
                                    blocks.push(block);
                                }
                            });
                        }
                        match config_kind {
                            GraphKind::Constructor => {
                                walker.for_all(|walker| {
                                    walker.node.attributes["isConstructor"].as_bool().unwrap_or(false)
                                }, |walkers| {
                                    for walker in walkers.iter() {
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
                                    for walker in walkers.iter() {
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
                                    for walker in walkers.iter() {
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
                    let from = walker.node.source_offset as usize;
                    let to = from + walker.node.source_len as usize;
                    let source = &self.source[from..=to];
                    match props[index] {
                        "initializationExpression" => {
                            init = CodeBlock::Block(walker.to_block_content(self.source));
                        },
                        "condition" => {
                            condition = CodeBlock::Block(walker.to_block_content(self.source));
                        },
                        "loopExpression" => {
                            expression = CodeBlock::Block(walker.to_block_content(self.source));
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
                            condition = CodeBlock::Block(walker.to_block_content(self.source));
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
                            condition = CodeBlock::Block(walker.to_block_content(self.source));
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
                            condition = CodeBlock::Block(walker.to_block_content(self.source));
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
                self.root = self.build_node(NodeKind::Root, self.walker);
                &self.root
            },
            _ => &self.root,
        }
    }
}
