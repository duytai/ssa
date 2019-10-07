use crate::dfg::Network;
use crate::core::Action;
use crate::core::Member;
use crate::core::Shape;
use std::collections::HashMap;
use std::collections::HashSet;

/// How to check:
/// execution_path contains send/transfer/delegatecall/call/callcode/selfdestruct/suicide
/// block.number/block.timestamp is used in these functions
/// condition in execution_path use block.number/block.timestamp directly or depend on them 
///
/// block.numer is saved to variable
/// invoke function call
pub struct UnsafeSendingCondition {
    block_timestamps: HashSet<(u32, u32)>,
    block_numbers: HashSet<(u32, u32)>,
}

impl UnsafeSendingCondition {
    pub fn new(network: &Network) -> Self {
        let mut unsafe_sending_condition = UnsafeSendingCondition {
            block_timestamps: HashSet::new(),
            block_numbers: HashSet::new(),
        };
        unsafe_sending_condition
    }

    fn update(&mut self, network: &Network) {
        // let is_condition = |id: u32| -> bool {
            // for vertice in all_vertices.iter() {
                // let vertex_id = vertice.get_id();
                // let shape = vertice.get_shape();
                // if vertex_id == id {
                    // return shape == &Shape::Diamond || shape == &Shape::Star;
                // }
            // }
            // false
        // };

    }

    pub fn get_block_numbers(&self) -> &HashSet<(u32, u32)> {
        &self.block_numbers
    }

    pub fn get_block_timestamps(&self) -> &HashSet<(u32, u32)> {
        &self.block_timestamps
    }
}
