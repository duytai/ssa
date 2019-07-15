use crate::dfg::Network;
use crate::core::{
    Shape,
    Action,
    Member,
};

/// Detect gasless send <X>.send() / <X>.transfer()
/// - address <X> depends on parameters
///  + has no condition check on path to <X> 
/// - address <X> depends on msg.sender
///  + has no condition check on path to <X> 
pub struct GaslessSend<'a> {
    network: &'a Network<'a>,
}

impl<'a> GaslessSend <'a> {
    pub fn new(network: &'a Network<'a>) -> Self {
        GaslessSend { network }
    }

    pub fn run(&self) -> bool {
        let dfgs = self.network.get_dfgs();
        let send = Member::Global(String::from("send"));
        let transfer = Member::Global(String::from("transfer"));
        for (_, dfg) in dfgs {
            // Find send / transfer
            let vertices = dfg.get_cfg().get_vertices();
            let new_actions = dfg.get_new_actions();
            for vertice in vertices {
                // Functioncall node
                if vertice.get_shape() == &Shape::DoubleCircle {
                    let vertex_id = vertice.get_id();
                    if let Some(actions) = new_actions.get(&vertex_id) {
                        for action in actions {
                            if let Action::Use(var, _) = action {
                                let members = var.get_members();
                                if members.contains(&send) || members.contains(&transfer) {
                                    let paths = self.network.traverse(vertex_id);
                                    for links in paths {
                                        println!(">>>");
                                        for link in links {
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        true
    }
}
