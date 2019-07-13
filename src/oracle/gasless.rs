use crate::dfg::Network;
use crate::core::{
    Member,
    Action,
};

pub struct Gasless<'a> {
    network: &'a Network<'a>,
}

impl<'a> Gasless<'a> {
    pub fn new(network: &'a Network<'a>) -> Self {
        Gasless { network }
    }

    pub fn run(&self) -> bool {
        let dfgs = self.network.get_dfgs();
        let links = self.network.get_links();
        for (_, dfg) in dfgs {
            let actions = dfg.get_actions();
            for (_, actions) in  actions {
                for action in actions {
                    println!("action: {:?}", action);
                    match action {
                        Action::Use(var, id) => {
                            let members = var.get_members();
                            if !members.is_empty() {
                                let send = Member::Global(String::from("send"));
                                if members[0] == send {
                                    println!("find send() at line {}", id);
                                }
                            }
                        },
                        _ => {},
                    }
                }
            }
        }
        true
    }
}
