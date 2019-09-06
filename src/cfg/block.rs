use crate::core::Walker;

/// Block of source code
#[derive(Debug)]
pub enum CodeBlock<'a> {
    /// Keep the body of a function 
    Block(Walker<'a>),
    /// Keep primitive nodes 
    SimpleBlocks(Vec<SimpleBlockNode<'a>>),
    /// Keep compound nodes
    Link(Box<BlockNode<'a>>),
    /// Keep empty nodes
    None,
}

/// Primitive node
#[derive(Debug)]
pub enum SimpleBlockNode<'a> {
    Throw(Walker<'a>),
    Break(Walker<'a>),
    Continue(Walker<'a>),
    Require(Walker<'a>),
    Assert(Walker<'a>),
    Revert(Walker<'a>),
    Transfer(Walker<'a>),
    Suicide(Walker<'a>),
    Selfdestruct(Walker<'a>),
    ModifierInvocation(Walker<'a>),
    FunctionCall(Walker<'a>),
    Unit(Walker<'a>),
    None,
}

/// Compound node
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

/// Components of a while statement
#[derive(Debug)]
pub struct WhileStatement<'a> {
    /// While condition 
    pub condition: CodeBlock<'a>,
    /// While body
    pub blocks: Vec<CodeBlock<'a>>,
}

/// Components of a do while statement 
#[derive(Debug)]
pub struct DoWhileStatement<'a> {
    /// Do while condition
    pub condition: CodeBlock<'a>,
    /// Do while body
    pub blocks: Vec<CodeBlock<'a>>,
}

/// Components of a if statement
#[derive(Debug)]
pub struct IfStatement<'a> {
    /// If statement condition 
    pub condition: CodeBlock<'a>,
    /// True block body
    pub tblocks: Vec<CodeBlock<'a>>,
    /// False block body
    pub fblocks: Vec<CodeBlock<'a>>,
}

/// Components of a For statement
#[derive(Debug)]
pub struct ForStatement<'a> {
    /// For condition
    pub condition: CodeBlock<'a>,
    /// Initialize a variable
    pub init: CodeBlock<'a>,
    /// Comparison
    pub expression: CodeBlock<'a>,  
    /// Body of for statement
    pub blocks: Vec<CodeBlock<'a>>,
}
