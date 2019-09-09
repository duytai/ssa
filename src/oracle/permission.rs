use crate::dfg::Network;

pub struct Permission<'a> {
    network: &'a Network<'a>, 
}

impl<'a> Permission<'a> {
    pub fn new(network: &'a Network) -> Self {
        Permission { network }
    }
}
