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

    /// Find all childs including current node
    ///
    /// break_found: if walker encounters one node satisfing filter then it decides to continue
    /// traverse childrens of that node or not
    pub fn all_childs<Filter>(&self, break_found: bool, mut fi: Filter) -> Vec<Walker<'a>> where Filter: FnMut(&Walker) -> bool {
        let mut stacks = vec![self.clone()];
        let mut walkers = vec![];
        while !stacks.is_empty() {
            let item = stacks.pop().unwrap();
            if !break_found || !fi(&item) {
                for child in item.node.children.iter() {
                    let walker = Walker::new(child, item.source);
                    stacks.push(walker);
                }
            }
            if fi(&item) {
                walkers.insert(0, item);
            }
        }
        walkers
    }
}
