use super::{
    walker:: { Walker, Node }
};

#[derive(Debug)]
pub enum SymbolAction {
    Declare,
    Read,
    Write,
}

#[derive(Debug)]
pub struct Symbol {
    name: String,
    action: SymbolAction,
    depends: Vec<Symbol>,
}

#[derive(Debug)]
pub enum Link {
    Item(Symbol),
    More(usize),
}

#[derive(Debug)]
pub struct Table {
    symbols: Vec<Link>,
    parent: Option<usize>,
}

#[derive(Debug)]
pub struct SymbolTable {
    tables: Vec<Table>,
    head: Option<usize>, 
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable { tables: vec![], head: None }
    }

    pub fn insert(&mut self, symbol: Symbol) {
        if let Some(head) = self.head {
            self.tables[head].symbols.push(Link::Item(symbol));
        }
    }

    pub fn lookup(&self, name: String) -> Option<&Symbol> {
        None
    }

    pub fn enter_scope(&mut self) {
        let mut table = Table { symbols: vec![], parent: None };
        match self.head {
            Some(head) => {
                let link = Link::More(self.tables.len());
                self.tables[head].symbols.push(link);
                table.parent = Some(self.tables.len() - 1);
                self.head = Some(self.tables.len()); 
                self.tables.push(table);
            },
            None => {
                self.head = Some(0);
                self.tables.push(table);
            },
        }
    }

    pub fn exit_scope(&mut self) {
        if let Some(head) = self.head {
            self.head = self.tables[head].parent;
        }
    }

    pub fn digest(&mut self, walker: &Walker) {
        let Node { name, attributes, .. } = walker.node;
        match name {
            "VariableDeclaration" => {
                let var_name = attributes["name"]
                    .as_str()
                    .unwrap()
                    .to_string();
                let symbol = Symbol {
                    name: var_name,
                    action: SymbolAction::Declare,
                    depends: vec![],
                };
                self.insert(symbol);
            },
            "ParameterList" => {
                walker.for_each(|walker, _| {
                    let Node { attributes, .. } = walker.node;
                    let var_name = attributes["name"]
                        .as_str()
                        .unwrap()
                        .to_string();
                    let symbol = Symbol {
                        name: var_name,
                        action: SymbolAction::Declare,
                        depends: vec![],
                    };
                    self.insert(symbol);
                });
            },
            "ExpressionStatement" => {
                walker.all(|walker| {
                    walker.node.name == "Assignment"
                }, |walkers| {
                    for walker in walkers {
                        walker.all(|walker| {
                            walker.node.name == "Identifier"
                        }, |walkers| {
                            for walker in walkers {
                                println!("{}", walker.node.attributes["value"]);
                            }
                        });
                    }
                });
            },
            _ => {},
        }
    } 
}
