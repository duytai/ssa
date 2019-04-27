use crate::walker::{ Walker };
use std::collections::HashMap;

pub struct Attribute {
    name: String,
    kind: String,
} 

pub struct FunctionCall {
    name: String,
    params: Vec<String>,
    returns: Vec<String>,
}

pub enum Property {
    Attribute(Attribute),
    FunctionCall(FunctionCall),
}

pub struct GlobalTable {
    table: HashMap<String, Property>,
}


impl GlobalTable {
    pub fn new(value: &json::JsonValue) -> Self {
        GlobalTable { table: HashMap::new() }
    }
}
