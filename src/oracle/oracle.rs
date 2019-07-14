use crate::dfg::Network;
use crate::oracle::Gasless;

pub enum OracleAction {
    GaslessSend(u32),
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
            OracleAction::GaslessSend(entry_id) => {
                self.network.find_links(entry_id);
                let gasless = Gasless::new(&self.network); 
                gasless.run();
            },
        }
    }

    pub fn format(&self) -> String {
        self.network.format()
    }
}
