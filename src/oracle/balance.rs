use crate::dfg::Network;

pub struct Balance {
}

impl Balance {
    pub fn new(network: &Network) -> Self {
        let mut balance = Balance {};
        balance.update(network);
        balance
    }

    fn update(&mut self, network: &Network) {
    }
}
