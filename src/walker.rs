use json;

#[derive(Debug)]
pub struct Node<'a> {
    pub id: u32,
    pub level: u32,
    pub name: &'a str,
    pub src: &'a str,
    pub value: &'a json::JsonValue,
}

#[derive(Debug)]
pub struct Walker<'a> {
    nodes: Vec<Node<'a>>,
}

impl<'a> Walker<'a> {
    pub fn new(value: &'a json::JsonValue) -> Self {
        Walker { nodes: Walker::parse(value, 0) }
    }

    pub fn parse(value: &json::JsonValue, level: u32) -> Vec<Node> {
        let id = value["id"].as_u32().unwrap();
        let name = value["name"].as_str().unwrap();
        let src = value["src"].as_str().unwrap();
        let mut nodes = vec![ Node { id, name, src, value, level }];
        for child in value["children"].members() {
            nodes.append(&mut Walker::parse(child, level + 1));
        }
        nodes
    }

    pub fn for_each<F>(&self, cb: F) where F: Fn(&Node) {
        for node in &self.nodes {
            cb(node)
        }
    }
}
