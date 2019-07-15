use crate::dot::Dot;
use crate::cfg::ControlFlowGraph;
use crate::dfg::DataFlowGraph;
use crate::core::{
    DataLink,
    DataLinkLabel,
    Dictionary,
    Member,
    Variable,
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

    pub fn get_links(&self) -> &HashSet<DataLink> {
        &self.links
    }

    pub fn get_dfgs(&self) -> &HashMap<u32, DataFlowGraph> {
        &self.dfgs
    }

    pub fn get_dict(&self) -> &Dictionary {
        &self.dict
    }

    pub fn find_external_links(&mut self, entry_id: u32) {
        for walker in self.dict.lookup_functions(entry_id) {
            let function_calls = self.dict.lookup_function_calls(walker.node.id);
            for walker in function_calls.iter() {
                let walkers = walker.direct_childs(|_| true);
                let reference = walkers[0].node.attributes["referencedDeclaration"].as_u32();
                let source = walkers[0].node.source;
                let fc_id = walker.node.id;
                match reference {
                    // User defined functions
                    // Connect invoked_parameters to defined_parameters 
                    // Connect function_call to return statement
                    Some(reference) => {
                        for walker in self.dict.lookup_returns(reference) {
                            let members = vec![Member::Reference(walker.node.id)];
                            let variable = Variable::new(members, source.to_string());
                            let label = DataLinkLabel::InFrom(fc_id);
                            let link = DataLink::new_with_label(fc_id, walker.node.id, variable, label);
                            self.links.insert(link);
                        }
                        let defined_parameters = self.dict.lookup_parameters(reference);
                        let invoked_parameters = self.dict.lookup_parameters(fc_id);
                        for i in 0..defined_parameters.len() {
                            let defined_parameter = defined_parameters[i];
                            let invoked_parameter = invoked_parameters[i];
                            let members = vec![Member::Reference(invoked_parameter.node.id)];
                            let variable = Variable::new(members, defined_parameter.node.source.to_string());
                            let label = DataLinkLabel::OutTo(fc_id);
                            let link = DataLink::new_with_label(defined_parameter.node.id, invoked_parameter.node.id, variable, label);
                            self.links.insert(link);
                        }
                    },
                    // Global functions
                    // Connect function_calls to parameters
                    None => {
                        let invoked_parameters = self.dict.lookup_parameters(fc_id);
                        for invoked_parameter in invoked_parameters {
                            let members = vec![Member::Reference(invoked_parameter.node.id)];
                            let variable = Variable::new(members, invoked_parameter.node.source.to_string());
                            let link = DataLink::new(fc_id, invoked_parameter.node.id, variable);
                            self.links.insert(link);
                        }
                    },
                };
            }
        }
    } 

    pub fn find_internal_links(&mut self, entry_id: u32) {
        for walker in self.dict.lookup_functions(entry_id) {
            let cfg = ControlFlowGraph::new(self.dict, walker.node.id);
            self.dot.add_cfg(&cfg);
            let mut dfg = DataFlowGraph::new(cfg);
            self.links.extend(dfg.find_links());
            self.dfgs.insert(walker.node.id, dfg);
        }
    }

    pub fn find_links(&mut self, entry_id: u32) {
        self.find_internal_links(entry_id);
        self.find_external_links(entry_id);
        self.dot.add_links(&self.links);
    }

    pub fn format(&self) -> String {
        self.dot.format()
    }
}
