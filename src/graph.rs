use super::walker::{ Walker };

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
        });
    }
}
