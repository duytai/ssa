use json;

#[derive(Debug, Clone)]
pub struct Node<'a> {
    pub id: u32,
    pub name: &'a str,
    pub source: &'a str,
    pub attributes: &'a json::JsonValue,
    children: Vec<&'a json::JsonValue>,
}

#[derive(Debug, Clone)]
pub struct Walker<'a> {
    pub node: Node<'a>,
    pub source: &'a str,
}

impl<'a> Walker<'a> {
    pub fn new(value: &'a json::JsonValue, source: &'a str) -> Self {
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
        let from = src[0] as usize;
        let to = from + src[1] as usize;
        let node = Node {
            id,
            name,
            source: &source[from..to],
            attributes: &value["attributes"],
            children,
        };
        Walker { node, source }
    }

    pub fn for_each<Callback>(&self, mut cb: Callback) where Callback: FnMut(Walker<'a>, usize) {
        for (index, child) in self.node.children.iter().enumerate() {
            cb(Walker::new(child, self.source), index);
        }
    }

    pub fn for_all<Callback, Filter>(&self, mut fi: Filter, mut cb: Callback)
        where
            Callback: FnMut(Vec<Walker<'a>>),
            Filter: FnMut(&Walker) -> bool 
    {
        let mut walkers = vec![];
        for child in self.node.children.iter() {
            let walker = Walker::new(child, self.source);
            if fi(&walker) {
                walkers.push(walker);
            }
        }
        cb(walkers);
    }

    pub fn all<Callback, Filter>(&self, mut fi: Filter, mut cb: Callback)
        where
            Callback: FnMut(Vec<Walker<'a>>),
            Filter: FnMut(&Walker) -> bool
    {
        let mut stacks = vec![];
        let mut walkers = vec![];
        for child in self.node.children.iter() {
            let walker = Walker::new(child, self.source);
            stacks.push(walker);
        }
        while !stacks.is_empty() {
            let item = stacks.pop().unwrap();
            for child in item.node.children.iter() {
                let walker = Walker::new(child, item.source);
                stacks.push(walker);
            }
            if fi(&item) {
                walkers.insert(0, item);
            }
        }
        cb(walkers);
    }
}
