use crate::dot::Dot;
use crate::cfg::ControlFlowGraph;
use crate::dfg::DataFlowGraph;
use crate::core::{
    DataLink,
    FakeNode,
    Dictionary,
    ParameterOrder,
};
use std::collections::{
    HashMap,
    HashSet,
};

pub struct Network<'a> {
    dict: &'a Dictionary<'a>,
    links: HashSet<DataLink>,
    dfgs: HashMap<u32, DataFlowGraph<'a>>,
    dot: Dot,
}

impl<'a> Network<'a> {
    pub fn new(dict: &'a Dictionary) -> Self {
        Network {
            dict,
            dot: Dot::new(),
            links: HashSet::new(),
            dfgs: HashMap::new(),
        }
    }

    pub fn find_links(&mut self, entry_id: u32) {
        let mut opens: HashSet<u32> = HashSet::new();
        for walker in self.dict.lookup_functions(entry_id) {
            let cfg = ControlFlowGraph::new(self.dict, walker.node.id);
            self.dot.add_cfg(&cfg);
            let mut dfg = DataFlowGraph::new(cfg);
            self.links.extend(dfg.find_links(None, None));
            opens.extend(dfg.get_opens());
            self.dfgs.insert(walker.node.id, dfg);
        }
        for open in opens {
            if let Some(walker) = self.dict.lookup(open) {
                let childs = walker.direct_childs(|_| true);
                let reference = &childs[0].node.attributes["referencedDeclaration"];
                if let Some(reference) = reference.as_u32() {
                    if let Some(dfg) = self.dfgs.get_mut(&reference) {
                        // Call to function defined at @reference
                        // Add fake data to Return statement of that function
                        // Add fake data to ParameterList
                        let fake_node = FakeNode::parse_one(walker, false);
                        let po = ParameterOrder::parse(walker, self.dict);
                        let ctx_returns = (open, fake_node.get_variables().clone());
                        let ctx_params = (open, po.get_variables().clone());
                        self.links.extend(dfg.find_links(Some(ctx_params), Some(ctx_returns)));
                    }
                }
            }
        }
        self.dot.add_links(&self.links);
    }

    pub fn format(&self) -> String {
        self.dot.format()
    }
}
