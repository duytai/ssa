use crate::core::Walker;
use crate::core::SmartContract;
use crate::core::SmartContractQuery;
use std::collections::HashMap;

/// Allow searching by node id 
#[derive(Debug)]
pub struct Dictionary<'a> {
    entries: HashMap<u32, Walker<'a>>,
    smart_contract: SmartContract,
}

impl<'a> Dictionary<'a> {
    /// Create new dictionary
    pub fn new(value: &'a json::JsonValue, sources: &'a HashMap<String, String>) -> Self {
        let mut dict = Dictionary {
            entries: HashMap::new(),
            smart_contract: SmartContract::new(), 
        };
        for (name, source) in sources {
            let ast_one = &value["sources"][name]["AST"];
            let walker = Walker::new(ast_one, source);
            dict.traverse(&walker);
        }
        let contract_walkers = dict.entries.iter().map(|(_, walker)| walker)
            .filter(|walker| walker.node.name == "ContractDefinition")
            .collect::<Vec<&Walker>>();
        dict.smart_contract.update(contract_walkers);
        dict
    }

    fn traverse(&mut self, walker: &Walker<'a>) {
        for walker in walker.direct_childs(|_| true).into_iter() {
            self.traverse(&walker);
            self.entries.insert(walker.node.id, walker);
        }
    }

    pub fn find_ids(&self, query: SmartContractQuery) -> Vec<u32> {
        self.smart_contract.find(query).unwrap_or(vec![])
    }

    pub fn find_walkers(&self, query: SmartContractQuery) -> Vec<&Walker> {
        self.smart_contract
            .find(query)
            .map(|ids| {
                ids.into_iter()
                   .map(|id| self.entries.get(&id).unwrap())
                   .collect::<Vec<&Walker>>()
            })
            .unwrap_or(vec![])
    }

    /// Find walker by node id
    pub fn walker_at(&self, id: u32) -> Option<&Walker> {
        self.entries.get(&id)
    }

    /// Filter by
    pub fn filter_by(&self, name: &str) -> Vec<&Walker>  {
        self.entries.iter()
            .map(|(_, walker)| walker)
            .filter(|walker| walker.node.name == name)
            .collect::<Vec<&Walker>>()
    }
}
