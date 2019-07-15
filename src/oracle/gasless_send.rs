use crate::dfg::Network;
use crate::core::{
    Shape,
};

pub struct GaslessSend<'a> {
    network: &'a Network<'a>,
}

impl<'a> GaslessSend <'a> {
    pub fn new(network: &'a Network<'a>) -> Self {
        GaslessSend { network }
    }

    pub fn run(&self) -> bool {
        let dfgs = self.network.get_dfgs();
        let links = self.network.get_links();
        for (_, dfg) in dfgs {
            // Find send / transfer
            let vertices = dfg.get_cfg().get_vertices();
            for v in vertices {
                // Function call
                if v.get_shape() == &Shape::DoubleCircle {
                }
            }
        }
        true
    }
}
