use crate::walker::Walker;

#[derive(Debug)]
pub enum CodeBlock<'a> {
    Block(Walker<'a>),
    Link(Box<BlockNode<'a>>),
    None,
}

#[derive(Debug)]
pub enum BlockNode<'a> {
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
