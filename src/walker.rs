use json;

#[derive(Debug)]
pub struct Node<'a> {
    pub id: u32,
    pub name: &'a str,
    pub source_offset: u32,
    pub source_len: u32,
    pub attributes: &'a json::JsonValue,
    pub children: Vec<&'a json::JsonValue>,
}

#[derive(Debug)]
pub struct Contract<'a> {
    pub name: String,
    pub node: Node<'a>,
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
                    let node = Walker::parse(children);
                    let contract_name = children["attributes"]["name"]
                        .as_str()
                        .unwrap()
                        .to_string();
                    contracts.push(Contract {
                        name: contract_name,
                        node,
                    });
                }
            }
        }
        Walker { contracts }
    }

    pub fn parse(value: &json::JsonValue) -> Node {
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
        Node {
            id,
            name,
            source_offset: src[0],
            source_len: src[1],
            attributes: &value["attributes"],
            children,
        }
    }

    pub fn for_each<F>(&self, mut cb: F) where F: FnMut(&Contract) {
        for contract in &self.contracts {
            cb(contract);
        }
    }
}
