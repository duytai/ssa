use crate::dfg::Network;
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

    pub fn run(&mut self, action: OracleAction) {
        match action {
            OracleAction::IntegerOverflow=> {
                let overflow = IntegerOverflow::new(&self.network);
                overflow.analyze();
            },
        } 
    }

    pub fn format(&mut self) -> String {
        self.network.format()
    }
}
