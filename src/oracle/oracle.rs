use std::collections::HashSet;
use crate::{
    flow::State,
};

pub trait Oracle {
    fn analyze(&mut self, state: &State);
}
