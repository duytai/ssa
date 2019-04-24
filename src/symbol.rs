#[derive(Debug)]
pub enum SymbolAction {
    Declare,
    Read,
    Write,
}

#[derive(Debug)]
pub struct Symbol {
    name: String,
    kind: String,
    action: SymbolAction,
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

    pub fn insert(&mut self, name: String, kind: String, action: SymbolAction) {
        let symbol = Symbol { name, kind, action };
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
}
