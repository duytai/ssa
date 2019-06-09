use crate::walker::Walker;

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

impl<'a> CodeBlock<'a> {
    pub fn is_function_call(&self) -> bool {
        match self {
            CodeBlock::Block(walker) => {
                walker.node.name == "FunctionCall"
            },
            _ => false,
        }
    }

    pub fn find_function_calls(&self) -> Vec<CodeBlock<'a>> {
        match self {
            CodeBlock::Block(walker) => {
                let mut function_calls = vec![];
                walker.all(|walker| {
                    walker.node.name == "FunctionCall"
                }, |walkers| {
                    for walker in walkers {
                        walker.for_each(|walker, index| {
                            if index == 0 {
                                let function_name = walker.node.attributes["value"]
                                    .as_str()
                                    .unwrap_or("");
                                let block = CodeBlock::Block(walker);
                                match function_name {
                                    "revert" => {
                                        let node = GraphNode::Revert(block);
                                        function_calls.push(CodeBlock::Link(Box::new(node)));
                                    },
                                    "assert" => {
                                        let node = GraphNode::Assert(block);
                                        function_calls.push(CodeBlock::Link(Box::new(node)));
                                    },
                                    "require" => {
                                        let node = GraphNode::Require(block);
                                        function_calls.push(CodeBlock::Link(Box::new(node)));
                                    },
                                    "suicide" => {
                                        let node = GraphNode::Suicide(block);
                                        function_calls.push(CodeBlock::Link(Box::new(node)));
                                    },
                                    "selfdestruct" => {
                                        let node = GraphNode::Selfdestruct(block);
                                        function_calls.push(CodeBlock::Link(Box::new(node)));
                                    },
                                    _ => {
                                        let node = GraphNode::FunctionCall(block);
                                        function_calls.push(CodeBlock::Link(Box::new(node)));
                                    },
                                };
                            }
                        })
                    }
                });
                function_calls
            },
            _ => vec![],
        }
    }
}
