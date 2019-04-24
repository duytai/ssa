pub struct Flat<'a> {
    list: Vec<&'a json::JsonValue>,
}

impl<'a> Flat<'a> {
    pub fn new(value: &'a json::JsonValue) -> Self {
        Flat { list: Flat::traverse(value) }
    }

    pub fn traverse(value: &json::JsonValue) -> Vec<&json::JsonValue> {
        let mut list = vec![value];
        for child in value["children"].members() {
            list.append(&mut Flat::traverse(child));
        }
        list
    }

    pub fn lookup(self, id: u32) -> Option<&'a json::JsonValue> {
        for item in self.list {
            if item["id"].as_u32().unwrap_or(0) == id {
                return Some(item);
            }
        }
        None
    }
}
