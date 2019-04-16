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
pub struct Contract<'a> {
    pub name: String,
    nodes: Vec<Node<'a>>,
}

#[derive(Debug)]
pub struct Walker<'a> {
    contracts: Vec<Contract<'a>>,
}

impl<'a> Walker<'a> {
    pub fn new(value: &'a json::JsonValue) -> Self {
        let mut contracts: Vec<Contract> = vec![];
        for children in value["children"].members() {
            if let Some(name) = children["name"].as_str() {
                if name == "ContractDefinition" {
                    let nodes = Walker::parse(children, 0); 
                    let contract_name = children["attributes"]["name"]
                        .as_str()
                        .unwrap()
                        .to_string();
                    let contract = Contract::new(contract_name, nodes);
                    contracts.push(contract);
                }
            }
        }
        Walker { contracts }
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

    pub fn for_each<F>(&self, mut cb: F) where F: FnMut(&Contract) {
        for contract in &self.contracts {
            cb(contract)
        }
    }
}

impl<'a> Contract<'a> {
    pub fn new(name: String, nodes: Vec<Node<'a>>) -> Self {
        Contract { name, nodes }
    }

    pub fn for_each<F>(&self, mut cb: F) where F: FnMut(&Node, &json::JsonValue) {
        for node in &self.nodes {
            cb(node, node.value)
        }
    }
}
