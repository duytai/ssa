use crate::dfg::Network;
use crate::core::Walker;
use crate::oracle::IntegerOverflow;

pub enum OracleAction {
    IntegerOverflow,
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
            OracleAction::IntegerOverflow => IntegerOverflow::analyze(&self.network)
        } 
    }

    pub fn format(&mut self) -> String {
        self.network.format()
    }
}
