//! # ssa
//!
//! `ssa` is a collection of utillities to statically analyze solidity source codes
//!
//!
//!

extern crate cfg;
extern crate core;
extern crate dot;
extern crate dfg;
extern crate loader;

mod solidity {
    pub mod cfg {
        pub use cfg::*;
    }
    pub mod core {
        pub use core::*;
    }
    pub mod dot {
        pub use dot::*;
    }
    pub mod dfg {
        pub use dfg::*;
    }
    pub mod loader {
        pub use loader::*;
    }
}

pub use solidity::*;
