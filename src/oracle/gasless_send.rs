use crate::dfg::Network;
use crate::core::{
    Shape,
    Action,
    DataLinkLabel,
    DataLink,
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
        let dict = self.network.get_dict();
        let send = Member::Global(String::from("send"));
        let transfer = Member::Global(String::from("transfer"));
        for (_, dfg) in dfgs {
            // Find send / transfer
            let vertices = dfg.get_cfg().get_vertices();
            let new_actions = dfg.get_new_actions();
            for vertice in vertices {
                // Functioncall node
                let vertex_id = vertice.get_id();
                if let Some(actions) = new_actions.get(&vertex_id) {
                    for action in actions {
                        if let Action::Use(var, _) = action {
                            let members = var.get_members();
                            // Place where send()/transfer() occurrs 
                            if members.contains(&send) || members.contains(&transfer) {
                                let paths = self.network.traverse(vertex_id);
                                let address_paths: Vec<Vec<&DataLink>> = paths
                                    .into_iter()
                                    .filter(|path| {
                                        for link in path {
                                            match link.get_label() {
                                                DataLinkLabel::Internal => {
                                                    let ref_member = link
                                                        .get_var()
                                                        .get_members()
                                                        .iter()
                                                        .find(|m| match m {
                                                            Member::Reference(_) => true,
                                                            _ => false,
                                                        });
                                                    if let Some(Member::Reference(ref_id)) = ref_member {
                                                        let walker = dict.lookup(*ref_id).unwrap();
                                                        let variable_type = walker.node.attributes["type"].as_str();
                                                        if let Some(variable_type) = variable_type {
                                                            if !(variable_type.starts_with("struct") || variable_type.ends_with("[]") || variable_type == "address"){
                                                                return false;
                                                            }
                                                        } else {
                                                            return false;
                                                        }
                                                    } else {
                                                        return false;
                                                    }
                                                },
                                                DataLinkLabel::InFrom(_) => {},
                                                DataLinkLabel::OutTo(_) => {},
                                                DataLinkLabel::BuiltIn => {},
                                                DataLinkLabel::Executor => {},
                                            }
                                        }
                                        true
                                    }).collect();
                                // Check if
                                // + last link points to parameters 
                                // + last link points to msg.sender 
                                let satisfied_paths: Vec<Vec<&DataLink>> = address_paths
                                    .into_iter()
                                    .filter(|path| {
                                        let last_link = path.last().unwrap();
                                        println!("{:?}", last_link);
                                        true
                                    })
                                    .collect();
                                println!(">> satisfied_paths: {:?}", satisfied_paths);
                            }
                        }
                    }
                }
            }
        }
        true
    }
}
