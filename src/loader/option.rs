use std::path::Path;
use std::collections::HashMap;

pub enum SolidityOutputKind {
    AST,
}

pub struct SolidityASTOutput {
    pub ast: String,
    pub sources: HashMap<String, String>,
}

pub enum SolidityOutput {
    AST(SolidityASTOutput),
}

pub struct SolidityOption<'a> {
    pub bin_dir: &'a Path,
    pub contract: &'a Path,
    pub kind: SolidityOutputKind,
}
