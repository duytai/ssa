use crate::dfg::Network;

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
        let dict = self.network.get_dict();
    }

    pub fn format(&mut self) -> String {
        self.network.format()
    }
}
