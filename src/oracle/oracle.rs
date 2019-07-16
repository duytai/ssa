use crate::dfg::Network;
use crate::oracle::GaslessSend;

pub enum OracleAction {
    GaslessSend,
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
            OracleAction::GaslessSend => {
                GaslessSend::new(&self.network).run();
            },
        }
    }

    pub fn format(&self) -> String {
        self.network.format()
    }
}
