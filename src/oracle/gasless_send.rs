use crate::dfg::Network;

pub struct GaslessSend<'a> {
    network: &'a Network<'a>,
}

impl<'a> GaslessSend <'a> {
    pub fn new(network: &'a Network<'a>) -> Self {
        GaslessSend { network }
    }

    pub fn run(&self) -> bool {
        let dfgs = self.network.get_dfgs();
        let links = self.network.get_links();
        for (_, dfg) in dfgs {
            // Find send/transfer
        }
        true
    }
}
