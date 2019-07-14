use crate::dfg::Network;

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
        }
        true
    }
}
