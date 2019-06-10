use crate::walker::Walker;

#[derive(Debug)]
pub enum CodeBlock<'a> {
    Unit(Walker<'a>),
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
                "revert" => CodeBlock::Link(Box::new(GraphNode::Revert(block))), 
                "assert" => CodeBlock::Link(Box::new(GraphNode::Assert(block))),
                "require" => CodeBlock::Link(Box::new(GraphNode::Require(block))),
                "suicide" => CodeBlock::Link(Box::new(GraphNode::Suicide(block))),
                "selfdestruct" => CodeBlock::Link(Box::new(GraphNode::Selfdestruct(block))),
                _ => CodeBlock::Link(Box::new(GraphNode::FunctionCall(block))), 
            }
        })
    }

    pub fn split(&self) -> Vec<CodeBlock<'a>> {
        match self {
            CodeBlock::Block(walker) => {
                let mut calls = vec![];
                let mut last_call_source = None;
                walker.all(|walker| {
                    walker.node.name == "FunctionCall"
                }, |walkers| {
                    for walker in walkers {
                        let source = walker.node.source;
                        if let Some(call) = self.to_call(walker) {
                            calls.push(call);
                            last_call_source = Some(source);
                        }
                    }
                });
                if let Some(call) = self.to_call(walker.clone()) {
                    calls.push(call);
                } else if let Some(last_call_source) = last_call_source {
                    if last_call_source.trim() != walker.node.source.trim() {
                        calls.push(CodeBlock::Unit(walker.clone()));
                    }
                } else {
                    calls.push(CodeBlock::Unit(walker.clone()));
                }
                calls
            },
            _ => vec![],
        }
    }
}
