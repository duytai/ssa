use crate::dfg::Network;
use crate::core::Walker;
use crate::oracle::Permission;
use crate::oracle::Balance;

pub enum OracleAction {
    IntegerOverflow,
}

pub struct Oracle<'a> {
    network: Network<'a>,
}

impl<'a> Oracle<'a> {
    pub fn new(network: Network<'a>) -> Self {
        let permission = Permission::new(&network);
        for v in permission.get_owner_variables() {
            println!("owner_variable: {:?}", v);
        }
        let balance = Balance::new(&network);
        for v in balance.get_msg_value_variables() {
            println!("balance_variable: {:?}", v);
        }
        Oracle { network }
    }

    pub fn run(&mut self, action: OracleAction) -> Vec<(Walker, String)> {
        match action {
            OracleAction::IntegerOverflow => vec![], 
        } 
    }

    pub fn format(&mut self) -> String {
        self.network.format()
    }
}
