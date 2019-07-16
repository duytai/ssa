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
                let gasless_send = GaslessSend::new(&self.network);
                let paths = gasless_send.run();
                if paths.is_empty() {
                    println!(">> GaslessSend : None ");
                } else {
                    println!(">> GaslessSend : Found ");
                    for links in paths {
                        println!("  ++ Path ++");
                        for link in links {
                            println!("        {} => {}", link.get_from(), link.get_to());
                        }
                    }
                }
            },
        }
    }

    pub fn format(&self) -> String {
        self.network.format()
    }
}
