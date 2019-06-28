use crate::core::Dictionary;
use crate::cfg::ControlFlowGraph;

pub struct Network<'a> {
    dict: &'a Dictionary<'a>,
}

impl<'a> Network<'a> {
    pub fn new(dict: &'a Dictionary) -> Self {
        Network { dict }
    }

    pub fn find_links(&self, entry_id: u32) {
        for walker in self.dict.lookup_functions(entry_id) {
            // let cfg = ControlFlowGraph::new(self.dict);
            // cfg.start_at(walker.node.id);
        }
    }
}
