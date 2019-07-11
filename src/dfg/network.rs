use crate::core::Dictionary;
use crate::dot::Dot;
use crate::cfg::ControlFlowGraph;
use crate::dfg::DataFlowGraph;
use crate::core::DataLink;
use std::collections::{
    HashMap,
    HashSet,
};

pub struct Network<'a> {
    dict: &'a Dictionary<'a>,
    dot: Dot,
}

impl<'a> Network<'a> {
    pub fn new(dict: &'a Dictionary) -> Self {
        Network { dict, dot: Dot::new() }
    }

    pub fn find_links(&mut self, entry_id: u32) {
        let mut dfgs = HashMap::new();
        let mut opens: HashSet<u32> = HashSet::new();
        let mut links: HashSet<DataLink> = HashSet::new();
        for walker in self.dict.lookup_functions(entry_id) {
            let cfg = ControlFlowGraph::new(self.dict, walker.node.id);
            self.dot.add_cfg(&cfg);
            let mut dfg = DataFlowGraph::new(cfg);
            links.extend(dfg.find_links());
            opens.extend(dfg.get_opens());
            dfgs.insert(walker.node.id, dfg);
        }
        for open in opens {
            if let Some(walker) = self.dict.lookup(open) {
                let childs = walker.direct_childs(|_| true);
                let reference = &childs[0].node.attributes["referencedDeclaration"];
                if let Some(reference) = reference.as_u32() {
                    if let Some(dfg) = dfgs.get_mut(&reference) {
                        let returns = dfg.get_returns();
                        println!("returns: {:?}", returns);
                    }
                }
            }
        }
        self.dot.add_links(&links);
    }

    pub fn format(&self) -> String {
        self.dot.format()
    }
}
