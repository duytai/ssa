use crate::core::Walker;
use crate::cfg::{
    CodeBlock,
    BlockNode,
    SimpleBlockNode,
    IfStatement,
    WhileStatement,
    DoWhileStatement,
    ForStatement,
};

/// Process AST tree
///
/// This module reads AST tree and preprocess data for ControlFlowGraph, Graph separates a function
/// call to 2 blocks by calling `build_block`.
///
/// - Param 
/// - Body
///
/// For body, Graph calls `build_items` to find control flow tokens. For each token, Graph calls
/// `build_node` to find its components
///
/// It is noted that, Graph start at `build_node` with NodeKind::Root
#[derive(Debug)]
pub struct Graph<'a> {
    walker: Walker<'a>,
    root: BlockNode<'a>,
}

/// Kind of a graph node
#[derive(Debug)]
pub enum BlockKind {
    /// Parameter list of a function
    Param,
    /// Body of a function
    Body,
}

/// Kind of node in graph
///
/// It is detected based on token kind of AST
#[derive(Debug)]
pub enum NodeKind {
    /// Root of a function
    Root,
    /// IfStatement token
    IfStatement,
    /// WhileStatement token
    WhileStatement,
    /// ForStatement token
    ForStatement,
    /// DoWhileStatement token
    DoWhileStatement,
}

impl<'a> Graph<'a> {
    pub fn new(walker: Walker<'a>) -> Self {
        Graph { walker, root: BlockNode::None }
    }

    /// Find all nested function_calls and try to create a single node for each
    ///
    /// Some functions directly affect to control flow will be collected to precisely build cfg
    pub fn split(walker: Walker<'a>) -> Vec<SimpleBlockNode<'a>> {
        let mut function_calls = vec![];
        let ig = |_: &Walker, _: &Vec<Walker>| false;
        let fi = |walker: &Walker, _: &Vec<Walker>| {
            walker.node.name == "FunctionCall" || walker.node.name == "ModifierInvocation"
        };
        // Split parameters to other nodes
        for walker in walker.walk(true, ig, fi).into_iter() {
            for walker in walker.direct_childs(|_| true).into_iter() {
                function_calls.append(&mut Graph::split(walker));
            }
            let child_walkers = walker.direct_childs(|_| true);
            let function_name = child_walkers[0].node.attributes["value"].as_str();
            match function_name {
                Some(function_name) => match function_name {
                    "revert" => {
                        let node = SimpleBlockNode::Revert(walker);
                        function_calls.push(node);
                    },
                    "assert" => {
                        let node = SimpleBlockNode::Assert(walker);
                        function_calls.push(node);
                    },
                    "require" => {
                        let node = SimpleBlockNode::Require(walker);
                        function_calls.push(node);
                    },
                    "suicide" => {
                        let node = SimpleBlockNode::Suicide(walker);
                        function_calls.push(node);
                    },
                    "selfdestruct" => {
                        let node = SimpleBlockNode::Selfdestruct(walker);
                        function_calls.push(node);
                    },
                    _ => match walker.node.name {
                        "ModifierInvocation" => {
                            let node = SimpleBlockNode::ModifierInvocation(walker);
                            function_calls.push(node);
                        },
                        _ => {
                            let node = SimpleBlockNode::FunctionCall(walker);
                            function_calls.push(node);
                        }
                    }
                },
                None => {
                    let member_name = child_walkers[0].node.attributes["member_name"].as_str();
                    let reference = child_walkers[0].node.attributes["referencedDeclaration"].as_u32();
                    match (member_name, reference) {
                        (Some("transfer"), None) => {
                            let node = SimpleBlockNode::Transfer(walker);
                            function_calls.push(node);
                        },
                        (_, _) => {
                            let node = SimpleBlockNode::FunctionCall(walker);
                            function_calls.push(node);
                        },
                    }
                }
            }
        }
        if walker.node.name != "FunctionCall" && walker.node.name != "ModifierInvocation" {
            let node = SimpleBlockNode::Unit(walker.clone());
            function_calls.push(node);
        }
        function_calls
    }

    /// Traverse the body of a function based on token kind, for some special token call build_node
    /// to find it's components
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
                let node = BlockNode::Return(Graph::split(walker));
                vec![CodeBlock::Link(Box::new(node))]
            },
            "Throw" => {
                let node = SimpleBlockNode::Throw(walker);
                vec![CodeBlock::SimpleBlocks(vec![node])]
            },
            "Continue" => {
                let node = SimpleBlockNode::Continue(walker);
                vec![CodeBlock::SimpleBlocks(vec![node])]
            },
            "Break" => {
                let node = SimpleBlockNode::Break(walker);
                vec![CodeBlock::SimpleBlocks(vec![node])]
            },
            "VariableDeclarationStatement" | "EmitStatement" | "ExpressionStatement" | "PlaceholderStatement" => {
                vec![CodeBlock::Block(walker)]
            },
            "InlineAssemblyStatement" => unimplemented!(),
            _ => vec![CodeBlock::Block(walker)],
        }
    }

    /// Traverse parameter list and modifier invocations, call build_items to traverse body of a
    /// function
    pub fn build_block(&mut self, kind: BlockKind, walker: Walker<'a>) -> Vec<CodeBlock<'a>> {
        let mut blocks = vec![];
        match kind {
            BlockKind::Body => {
                for walker in walker.direct_childs(|_| true) {
                    blocks.append(&mut self.build_items(walker));
                }
            },
            BlockKind::Param => {
                for (index, walker) in walker.direct_childs(|_| true).into_iter().enumerate() {
                    match walker.node.name {
                        "ParameterList" => {
                            if index == 0 {
                                for walker in walker.direct_childs(|_| true) {
                                    let block = CodeBlock::Block(walker);
                                    blocks.push(block);
                                }
                            }
                        },
                        "Block" => {
                            blocks.append(&mut self.build_block(BlockKind::Body, walker));
                        },
                        "ModifierInvocation" => {
                            blocks.push(CodeBlock::Block(walker));
                        },
                        _ => {},
                    }
                }
            },
        }
        blocks
    } 

    /// For each node, try to detect it's components
    pub fn build_node(&mut self, kind: NodeKind, walker: Walker<'a>) -> BlockNode<'a> {
        match kind {
            NodeKind::Root => {
                BlockNode::Root(self.build_block(BlockKind::Param, walker))
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
                for (index, walker) in walker.direct_childs(|_| true).into_iter().enumerate() {
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
                }
                BlockNode::ForStatement(ForStatement { condition, init, expression, blocks })
            },
            NodeKind::DoWhileStatement => {
                let mut condition = CodeBlock::None; 
                let mut blocks = vec![];
                for (index, walker) in walker.direct_childs(|_| true).into_iter().enumerate() {
                    match index {
                        0 => {
                            condition = CodeBlock::Block(walker);
                        },
                        1 => match walker.node.name {
                            "Block" => {
                                blocks = self.build_block(BlockKind::Body, walker);
                            },
                            _ => {
                                blocks.append(&mut self.build_items(walker));
                            }
                        },
                        _ => unimplemented!(),
                    }
                }
                BlockNode::DoWhileStatement(DoWhileStatement { condition, blocks })
            },
            NodeKind::WhileStatement => {
                let mut condition = CodeBlock::None; 
                let mut blocks = vec![];
                for (index, walker) in walker.direct_childs(|_| true).into_iter().enumerate() {
                    match index {
                        0 => {
                            condition = CodeBlock::Block(walker);
                        },
                        1 => match walker.node.name {
                            "Block" => {
                                blocks = self.build_block(BlockKind::Body, walker);
                            },
                            _ => {
                                blocks.append(&mut self.build_items(walker));
                            }
                        },
                        _ => unimplemented!(),
                    }
                }
                BlockNode::WhileStatement(WhileStatement { condition, blocks })
            },
            NodeKind::IfStatement => {
                let mut condition = CodeBlock::None; 
                let mut tblocks = vec![];
                let mut fblocks = vec![];
                for (index, walker) in walker.direct_childs(|_| true).into_iter().enumerate() {
                    match index {
                        0 => {
                            condition = CodeBlock::Block(walker);
                        },
                        1 => match walker.node.name {
                            "Block" => {
                                tblocks = self.build_block(BlockKind::Body, walker);
                            },
                            _ => {
                                tblocks.append(&mut self.build_items(walker));
                            }
                        },
                        2 => match walker.node.name {
                            "Block" => {
                                fblocks = self.build_block(BlockKind::Body, walker);
                            },
                            _ => {
                                fblocks.append(&mut self.build_items(walker));
                            }
                        },
                        _ => unimplemented!(),
                    }
                }
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
