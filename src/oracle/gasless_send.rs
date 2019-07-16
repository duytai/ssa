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

    fn get_parameter_ids(&self) -> Vec<u32> {
        let mut ret = vec![];
        let dict = self.network.get_dict();
        for walker in dict.lookup_functions(self.network.get_entry_id()) {
            for walker in dict.lookup_parameters(walker.node.id) {
                ret.push(walker.node.id);
            }
        }
        ret
    }

    fn find_from(&self, vertex_id: u32) -> Vec<Vec<&DataLink>> {
        let dfgs = self.network.get_dfgs();
        let dict = self.network.get_dict();
        let parameter_ids = self.get_parameter_ids();
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
        // + address is set to state => state is used to send()/transfer()
        let satisfied_paths: Vec<Vec<&DataLink>> = address_paths
            .into_iter()
            .filter(|path| {
                let link_to = path.last().unwrap().get_to();
                // Address is provided in parameters
                if parameter_ids.contains(&link_to) {
                    return true;
                } else {
                    for (_, dfg) in dfgs {
                        if let Some(actions) = dfg.get_new_actions().get(&link_to) {
                            for action in actions {
                                if let Action::Use(var, _) = action {
                                    let msg = Member::Global(String::from("msg"));
                                    let sender = Member::Global(String::from("sender"));
                                    // msg.sender is assigned to variable
                                    if var.get_members() == &vec![sender, msg] {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                    return false;
                }
            })
        .collect();
        satisfied_paths
    }

    pub fn run(&self) -> Vec<Vec<&DataLink>> {
        let mut satisfied_paths = vec![];
        let dfgs = self.network.get_dfgs();
        let send = Member::Global(String::from("send"));
        let transfer = Member::Global(String::from("transfer"));
        for (_, dfg) in dfgs {
            // Find send / transfer
            let vertices = dfg.get_cfg().get_vertices();
            for vertice in vertices {
                // Functioncall node
                let vertex_id = vertice.get_id();
                if let Some(actions) = dfg.get_new_actions().get(&vertex_id) {
                    for action in actions {
                        if let Action::Use(var, _) = action {
                            let members = var.get_members();
                            // Place where send()/transfer() occurrs 
                            if members.contains(&send) || members.contains(&transfer) {
                                satisfied_paths.append(&mut self.find_from(vertex_id));
                            }
                        }
                    }
                }
            }
        }
        satisfied_paths
    }
}
