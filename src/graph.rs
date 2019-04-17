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
pub enum CodeBlock {
    Block(String),
    Link(Box<GraphNode>),
    None,
}

#[derive(Debug)]
pub enum GraphNode {
    Standard(Vec<CodeBlock>),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    ForStatement(ForStatement),
    DoWhileStatement(DoWhileStatement),
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
    condition: CodeBlock,
    tblocks: Vec<CodeBlock>,
    fblocks: Vec<CodeBlock>,
}

#[derive(Debug)]
pub struct ForStatement {
    first: CodeBlock,
    second: CodeBlock,
    third: CodeBlock,
    blocks: Vec<CodeBlock>,
}

impl<'a> Graph<'a> {
    pub fn new(walker: &'a Walker, source: &'a str) -> Self {
        Graph { walker, source, root: GraphNode::None }
    }

    pub fn build_block(&mut self, kind: BlockKind, walker: &Walker) -> Vec<CodeBlock> {
        let mut blocks = vec![];
        let mut is_return = false;
        match kind {
            BlockKind::BlockBody => {
                walker.for_each(|walker, _| {
                    if is_return { return; }
                    match walker.node.name {
                        "IfStatement" => {
                            let node = self.build_node(NodeKind::IfStatement, walker); 
                            let block = CodeBlock::Link(Box::new(node));
                            blocks.push(block);
                        },
                        "WhileStatement" => {
                            let node = self.build_node(NodeKind::WhileStatement, walker);
                            let block = CodeBlock::Link(Box::new(node));
                            blocks.push(block);
                        },
                        "ForStatement" => {
                            let node = self.build_node(NodeKind::ForStatement, walker);
                            let block = CodeBlock::Link(Box::new(node));
                            blocks.push(block);
                        },
                        "DoWhileStatement" => {
                            let node = self.build_node(NodeKind::DoWhileStatement, walker);
                            let block = CodeBlock::Link(Box::new(node));
                            blocks.push(block);
                        },
                        _ => {
                            let from = walker.node.source_offset as usize;
                            let to = from + walker.node.source_len as usize;
                            let source = &self.source[from..to];
                            let block = CodeBlock::Block(source.to_string());
                            blocks.push(block);
                            if walker.node.name == "Return" {
                                is_return = true;
                            }
                        }
                    }
                })
            },
            BlockKind::Constructor => {
                walker.for_each(|walker, index| {
                    if walker.node.name == "ParameterList" && index == 0 {
                        let from = walker.node.source_offset as usize;
                        let to = from + walker.node.source_len as usize;
                        let source = &self.source[from..to];
                        let block = CodeBlock::Block(source.to_string());
                        blocks.push(block);
                    }
                    if walker.node.name == "Block" {
                        blocks.append(&mut self.build_block(BlockKind::BlockBody, walker));
                    }
                })
            },
            _ => {},
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
                                    let block = CodeBlock::Block(source.to_string());
                                    state_blocks.push(block);
                                }
                            }
                        });
                    }
                });
                blocks.append(&mut state_blocks);
                blocks.append(&mut constructor_blocks);
                GraphNode::Standard(blocks)
            },
            NodeKind::ForStatement => {
                let mut first = CodeBlock::None;
                let mut second = CodeBlock::None;
                let mut third = CodeBlock::None;
                let mut blocks = vec![];
                walker.for_each(|walker, index| {
                    match index {
                        0 => {
                            let from = walker.node.source_offset as usize;
                            let to = from + walker.node.source_len as usize;
                            let source = &self.source[from..=to];
                            first = CodeBlock::Block(source.to_string());
                        },
                        1 => {
                            let from = walker.node.source_offset as usize;
                            let to = from + walker.node.source_len as usize;
                            let source = &self.source[from..=to];
                            second = CodeBlock::Block(source.to_string());
                        },
                        2 => {
                            let from = walker.node.source_offset as usize;
                            let to = from + walker.node.source_len as usize;
                            let source = &self.source[from..=to];
                            third = CodeBlock::Block(source.to_string());
                        },
                        _ => {
                            if walker.node.name == "Block" {
                                blocks = self.build_block(BlockKind::BlockBody, walker);
                            }
                        },
                    }
                });
                GraphNode::ForStatement(ForStatement { first, second, third, blocks })
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
                            condition = CodeBlock::Block(source.to_string());
                        },
                        "Block" => {
                            blocks = self.build_block(BlockKind::BlockBody, walker);
                        },
                        _ => {},
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
                            condition = CodeBlock::Block(source.to_string());
                        },
                        "Block" => {
                            blocks = self.build_block(BlockKind::BlockBody, walker);
                        },
                        _ => {},
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
                            condition = CodeBlock::Block(source.to_string());
                        },
                        "Block" => {
                            if index == 1 {
                                tblocks = self.build_block(BlockKind::BlockBody, walker);
                            } else {
                                fblocks = self.build_block(BlockKind::BlockBody, walker);
                            }
                        },
                        _ => {},
                    }
                });
                GraphNode::IfStatement(IfStatement { condition, tblocks, fblocks })
            },
            _ => GraphNode::None,
        } 
    }

    pub fn build(&mut self) {
        self.root = self.build_node(NodeKind::Root, self.walker);
        println!("{:?}", self.root);
    }
}
