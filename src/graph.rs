use super::walker::{ Walker };
use super::components::{ StateVariableDeclaration };

pub struct Graph<'a> {
    walker: &'a Walker<'a>,
    source: &'a str,
}

impl<'a> Graph<'a> {
    pub fn new(walker: &'a Walker, source: &'a str) -> Self {
        Graph { walker, source }
    }

    pub fn build(&self) {
        self.walker.for_each(|contract| {
            let mut state = StateVariableDeclaration::new();
            contract.for_each(|node| {
                state.visit(node, node.value);
            });
            println!("state: {:?}", state.report(self.source));
        });
    }
}
