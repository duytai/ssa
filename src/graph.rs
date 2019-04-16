use super::walker::{ Walker };

pub struct Graph<'a> {
    walker: &'a Walker<'a>,
    source: &'a str,
}

impl<'a> Graph<'a> {
    pub fn new(walker: &'a Walker, source: &'a str) -> Self {
        Graph { walker, source }
    }

    pub fn build(&mut self) {
        self.walker.for_each(|walker| {
            if walker.node.name == "ContractDefinition" {
                // Read a contract
                walker.for_each(|walker| {
                    match walker.node.name {
                        "FunctionDefinition" => {},
                        _ => {},
                    }
                });
            }
        });
    }
}
