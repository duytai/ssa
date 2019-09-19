use crate::dfg::Network;
use crate::core::{
    Variable,
    Member,
};

pub struct Permission<'a> {
    network: &'a Network<'a>, 
}

impl<'a> Permission<'a> {
    pub fn new(network: &'a Network) -> Self {
        let variable = Variable::new(
            vec![Member::Reference(18)],
            String::from("z"),
            String::from("uint256"),
        );
        network.traverse(30, &variable);
        Permission { network }
    }
}
