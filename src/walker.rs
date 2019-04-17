use json;

#[derive(Debug)]
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

    pub fn len(&self) -> usize {
        self.node.children.len()
    }

    pub fn for_each<F>(&self, mut cb: F) where F: FnMut(&Walker, usize) {
        for (index, child) in self.node.children.iter().enumerate() {
            cb(&Walker::new(child), index);
        }
    }
}
