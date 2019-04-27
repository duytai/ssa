use crate::walker::{ Walker, Node };

#[derive(Debug)]
pub enum LocalSymbolAction {
    Definition,
    Declaration,
    Reading,
    Writing,
}

#[derive(Debug)]
pub struct LocalSymbol {
    name: String,
    action: LocalSymbolAction,
}

#[derive(Debug)]
pub enum Link {
    Item(LocalSymbol),
    More(usize),
}

#[derive(Debug)]
pub struct Table {
    symbols: Vec<Link>,
    parent: Option<usize>,
}

#[derive(Debug)]
pub struct LocalTable {
    tables: Vec<Table>,
    head: Option<usize>, 
}

impl LocalTable {
    pub fn new() -> Self {
        LocalTable { tables: vec![], head: None }
    }

    pub fn insert(&mut self, symbol: LocalSymbol) {
        if let Some(head) = self.head {
            self.tables[head].symbols.push(Link::Item(symbol));
        }
    }

    pub fn lookup(&self, name: String) -> Option<&LocalSymbol> {
        unimplemented!();
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

    pub fn digest(&mut self, _walker: &Walker) {
    } 
}
