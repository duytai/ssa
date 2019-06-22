use std::collections::HashMap;
use json;

/// AST node
#[derive(Debug, Clone)]
pub struct Node<'a> {
    pub id: u32,
    pub name: &'a str,
    pub source: &'a str,
    pub attributes: &'a json::JsonValue,
    children: Vec<&'a json::JsonValue>,
}

/// A pointer to a node of AST tree 
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

    /// Find all direct childrens
    pub fn direct_childs<Filter> (&self, fi: Filter) -> Vec<Walker<'a>> where Filter: Fn(&Walker) -> bool {
        let mut walkers = vec![];
        for child in self.node.children.iter() {
            let walker = Walker::new(child, self.source);
            if fi(&walker) {
                walkers.push(walker);
            }
        }
        walkers
    }

    pub fn walk<F, I>(&self, bf: bool, ig: I, fi: F) -> Vec<Walker<'a>>
        where
            F: Fn(&Walker, &Vec<Walker>) -> bool,
            I: Fn(&Walker, &Vec<Walker>) -> bool
    {
        let mut stacks = vec![self.clone()];
        let mut walkers = vec![];
        let mut paths = HashMap::new(); 
        paths.insert(self.node.id, vec![self.clone()]);
        while !stacks.is_empty() {
            let item = stacks.pop().unwrap();
            if !ig(&item, &paths[&item.node.id]) {
                if !(fi(&item, &paths[&item.node.id]) && bf) {
                    for child in item.node.children.iter() {
                        let mut path = paths[&item.node.id].clone();
                        let walker = Walker::new(child, item.source);
                        path.push(walker.clone());
                        paths.insert(walker.node.id, path);
                        stacks.push(walker);
                    }
                }
            }
            if fi(&item, &paths[&item.node.id]) {
                walkers.insert(0, item);
            }
        }
        walkers
    }
}
