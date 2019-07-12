use crate::dfg::Network;
use std::collections::HashSet;
use crate::core::{
    Member,
    Variable,
};

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
                let mut ctx_root: HashSet<Variable> = HashSet::new();
                let members = vec![
                    Member::Global(String::from("sender")),
                    Member::Global(String::from("msg")),
                ];
                let msg_sender = Variable::new(members, String::from("msg.sender"));
                ctx_root.insert(msg_sender);
                self.network.find_links(entry_id, Some(ctx_root));
            },
        }
    }

    pub fn format(&self) -> String {
        self.network.format()
    }
}
