use crate::walker::Walker;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Dictionary<'a> {
    entries: HashMap<u32, Walker<'a>>,
}

impl<'a> Dictionary<'a> {
    pub fn new(value: &'a json::JsonValue, source: &'a str) -> Self {
        let walker = Walker::new(value, source);
        let mut dict = Dictionary { entries: HashMap::new() };
        dict.traverse(&walker);
        dict
    }

    pub fn traverse(&mut self, walker: &Walker<'a>) {
        walker.for_each(|walker, _| {
            self.traverse(&walker);
            self.entries.insert(walker.node.id, walker);
        });
    }

    pub fn lookup(&self, id: u32) -> Option<&Walker> {
        self.entries.get(&id)
    }
}
