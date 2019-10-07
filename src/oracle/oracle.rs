use crate::dfg::Network;
use crate::core::Walker;
use crate::oracle::{
    UnsafeSendingCondition,
};

pub enum OracleAction {
    UnsafeSendingCondition,
    Suicide,
}

pub struct Oracle<'a> {
    network: Network<'a>,
}

impl<'a> Oracle<'a> {
    pub fn new(network: Network<'a>) -> Self {
        Oracle { network }
    }

    pub fn run(&mut self, action: OracleAction) -> Vec<(Walker, String)> {
        let dict = self.network.get_dict();
        match action {
            OracleAction::Suicide => {
                vec![]
            },
            OracleAction::UnsafeSendingCondition => {
                let unsafe_condition = UnsafeSendingCondition::new(&self.network);
                let block_numbers = unsafe_condition.get_block_numbers();
                let block_timestamps = unsafe_condition.get_block_timestamps();
                for (send_at, depend_at) in block_numbers {
                    let send_walker = dict.walker_at(*send_at).unwrap();
                    let depend_walker = dict.walker_at(*depend_at).unwrap();
                    println!(
                        "[WARNING] block_number_dependency\n\t{}\n\t{}",
                        depend_walker.node.source,
                        send_walker.node.source,
                    );
                }
                for (send_at, depend_at) in block_timestamps {
                    let send_walker = dict.walker_at(*send_at).unwrap();
                    let depend_walker = dict.walker_at(*depend_at).unwrap();
                    println!(
                        "[WARNING] block_timestamp_dependency\n\t{}\n\t{}",
                        depend_walker.node.source,
                        send_walker.node.source,
                    );
                }
                vec![]
            }, 
        } 
    }

    pub fn format(&mut self) -> String {
        self.network.format()
    }
}
