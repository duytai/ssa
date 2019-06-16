extern crate reqwest;

mod solidity; 
mod option;

pub use solidity::Solidity;
pub use option::{
    SolidityOption,
    SolidityOutputKind,
    SolidityOutput,
    SolidityASTOutput,
};
