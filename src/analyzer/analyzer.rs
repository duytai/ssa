use crate::{
    flow::State,
};

pub trait Analyzer {
    fn analyze(&mut self, state: &State);
}
