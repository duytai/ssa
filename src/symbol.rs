pub struct Symbol {
    name: String,
    kind: String,
}

pub struct SymbolTable {
    symbols: Vec<Symbol>,
    entries: Vec<usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable { symbols: vec![], entries: vec![] }
    }

    pub fn insert(&mut self, name: String, kind: String) {
        self.symbols.push(Symbol { name, kind });
    }

    pub fn lookup(&self, name: String) -> Option<&Symbol> {
        for i in (0..self.symbols.len()).rev() {
            let symbol = &self.symbols[i];
            if symbol.name == name { return Some(symbol); }
        }
        None
    }

    pub fn enter_scope(&mut self) {
        self.entries.push(self.symbols.len());
    }

    pub fn exit_scope(&mut self) {
        let entry = self.entries.pop().unwrap();
        self.symbols.truncate(entry);
    }
}
