use crate::walker::Walker;

#[derive(Debug)]
pub enum CodeBlock<'a> {
    Block(Walker<'a>),
    SimpleBlocks(Vec<SimpleBlockNode<'a>>),
    Link(Box<BlockNode<'a>>),
    None,
}

#[derive(Debug)]
pub enum SimpleBlockNode<'a> {
    Throw(Walker<'a>),
    Break(Walker<'a>),
    Continue(Walker<'a>),
    Require(Walker<'a>),
    Assert(Walker<'a>),
    Revert(Walker<'a>),
    Suicide(Walker<'a>),
    Selfdestruct(Walker<'a>),
    FunctionCall(Walker<'a>),
    Unit(Walker<'a>),
    None,
}

#[derive(Debug)]
pub enum BlockNode<'a> {
    Root(Vec<CodeBlock<'a>>),
    IfStatement(IfStatement<'a>),
    WhileStatement(WhileStatement<'a>),
    ForStatement(ForStatement<'a>),
    DoWhileStatement(DoWhileStatement<'a>),
    Return(Vec<SimpleBlockNode<'a>>),
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
