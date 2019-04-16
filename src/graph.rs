use super::walker::{ Walker };
use super::components::{ StateVariableDeclaration };

pub struct Graph<'a> {
    walker: &'a Walker<'a>,
}

impl<'a> Graph<'a> {
    pub fn new(walker: &'a Walker) -> Self {
        Graph { walker }
    }

    pub fn build(&self) {
        let state = StateVariableDeclaration::new();
        self.walker.for_each(|node| {
            state.visit(node)
        });
    }
}
