use crate::dfg::Network;
use crate::core::Walker;
use crate::oracle::UnsafeSendingCondition;

pub enum OracleAction {
    UnsafeSendingCondition,
}

pub struct Oracle<'a> {
    network: Network<'a>,
}

impl<'a> Oracle<'a> {
    pub fn new(network: Network<'a>) -> Self {
        Oracle { network }
    }

    pub fn run(&mut self, action: OracleAction) -> Vec<(Walker, String)> {
        match action {
            OracleAction::UnsafeSendingCondition => {
                let unsafe_condition = UnsafeSendingCondition::new(&self.network);
                vec![]
            }, 
        } 
    }

    pub fn format(&mut self) -> String {
        self.network.format()
    }
}
