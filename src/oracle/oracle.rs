use crate::dfg::Network;
use crate::oracle::{
    GaslessSend,
    GaslessSendResult,
};

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
                let gasless_send = GaslessSend::new(&self.network);
                let result = gasless_send.run();
                for r in result {
                    match r {
                        GaslessSendResult::DirectUse(v) => {
                            println!("Use: {}", v.get_source());
                        },
                        GaslessSendResult::LinkedUse(path) => {
                            for link in path {
                                println!("        {} => {}", link.get_from(), link.get_to());
                            }
                        },
                    }
                }
            },
        }
    }

    pub fn format(&self) -> String {
        self.network.format()
    }
}
