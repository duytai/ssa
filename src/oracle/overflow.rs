use crate::dfg::Network;
use crate::core::{
    Walker,
    Variable,
};

pub struct IntegerOverflow<'a> {
    network: &'a Network<'a>,
}

impl<'a> IntegerOverflow<'a> {
    pub fn new(network: &'a Network<'a>) -> Self {
        let mut overflow = IntegerOverflow { network };
        overflow
    }

    pub fn analyze(&self) {
        // Dictionary that we can lookup 
        let dict = self.network.get_dict();
        // List of data flow graph, one dfg for a function 
        let dfgs = self.network.get_dfgs();
        // id is id of FunctionDefinition node 
        for (id, _) in dfgs {
            if let Some(walker) = dict.lookup(*id) {
                // Search binary operator +
                let filter = |walker: &Walker, _: &Vec<Walker>| {
                    let operator = walker.node.attributes["operator"].as_str().unwrap_or("");
                    walker.node.name == "BinaryOperation" && operator == "+"
                };
                let ignore = |_: &Walker, _: &Vec<Walker>| false;
                // Expression contains + operator
                let walkers = walker.walk(false, ignore, filter);
                for walker in walkers {
                    // Expression: VarA + VarB 
                    let walkers = walker.direct_childs(|_| true);
                    for walker in walkers {
                        // Find variables in expression and detect its type
                        match Variable::parse_one(&walker, dict) {
                            Some (variable) => {
                                let variable_type = variable.get_type();
                                println!("variable: {:?}", variable);
                                println!("variable_type: {:?}", variable_type);
                            },
                            None => {
                                let variable_type = walker.node.attributes["type"].as_str();
                                println!("name: {}", walker.node.name);
                                println!("variable_type: {:?}", variable_type);
                            },
                        }
                    }
                }
            }
        }
    }
}

