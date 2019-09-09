use crate::dfg::Network;
use crate::core::Walker;
use crate::oracle::Permission;

pub enum OracleAction {
    IntegerOverflow,
}

pub struct Oracle<'a> {
    network: Network<'a>,
}

impl<'a> Oracle<'a> {
    pub fn new(network: Network<'a>) -> Self {
        let permission = Permission::new(&network);
        permission.create_table();
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
