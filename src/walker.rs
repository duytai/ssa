use json;

#[derive(Debug, Clone)]
pub struct Node<'a> {
    pub id: u32,
    pub name: &'a str,
    pub source_offset: u32,
    pub source_len: u32,
    pub attributes: &'a json::JsonValue,
    children: Vec<&'a json::JsonValue>,
}

#[derive(Debug)]
pub struct Walker<'a> {
    pub node: Node<'a>,
}

impl<'a> Walker<'a> {
    pub fn new(value: &'a json::JsonValue) -> Self {
        let id = value["id"].as_u32().unwrap();
        let name = value["name"].as_str().unwrap();
        let src = value["src"].as_str().unwrap();
        let src = src.split(":")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let mut children = vec![];
        for child in value["children"].members() {
            children.push(child);
        }
        let node = Node {
            id,
            name,
            source_offset: src[0],
            source_len: src[1],
            attributes: &value["attributes"],
            children,
        };
        Walker { node }
    }

    pub fn for_each<Callback>(&self, mut cb: Callback) where Callback: FnMut(&Walker, usize) {
        for (index, child) in self.node.children.iter().enumerate() {
            cb(&Walker::new(child), index);
        }
    }

    pub fn for_all<Callback, Filter>(&self, mut fi: Filter, mut cb: Callback)
        where
            Callback: FnMut(Vec<Walker>),
            Filter: FnMut(&Walker) -> bool 
    {
        let mut walkers = vec![];
        for child in self.node.children.iter() {
            let walker = Walker::new(child);
            if fi(&walker) {
                walkers.push(walker);
            }
        }
        cb(walkers);
    }

    pub fn all<Callback, Filter>(&self, mut fi: Filter, mut cb: Callback)
        where
            Callback: FnMut(Vec<Walker>),
            Filter: FnMut(&Walker) -> bool
    {
        let mut stacks = vec![];
        let mut walkers = vec![];
        for child in self.node.children.iter() {
            let walker = Walker::new(child);
            stacks.push(walker);
        }
        while !stacks.is_empty() {
            let item = stacks.pop().unwrap();
            for child in item.node.children.iter() {
                let walker = Walker::new(child);
                stacks.push(walker);
            }
            if fi(&item) {
                walkers.insert(0, item);
            }
        }
        cb(walkers);
    }
}
