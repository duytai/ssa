use crate::core::Walker;
use crate::core::SmartContract;
use crate::core::SmartContractQuery;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub enum LookupInputType<'a> {
    FunctionId(u32),
    FunctionCallId(u32),
    ContractId(u32),
    ContractName(&'a str),
}

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

    pub fn find(&self, query: SmartContractQuery) -> Option<&Vec<u32>> {
        self.smart_contract.find(query)
    }

    /// Find walker by node id
    pub fn lookup(&self, id: u32) -> Option<&Walker> {
        self.entries.get(&id)
    }

    /// Filter by
    pub fn filter_by(&self, name: &str) -> Vec<&Walker>  {
        self.entries.iter()
            .map(|(_, walker)| walker)
            .filter(|walker| walker.node.name == name)
            .collect::<Vec<&Walker>>()
    }

    /// Find contract constructor
    pub fn lookup_constructor(&self, lookup_input: LookupInputType) -> Option<Walker> {
        None
    }

    /// Find return statements
    pub fn lookup_returns(&self, id: u32) -> Vec<&Walker> {
        vec![]
    } 

    /// Find all parameters of a function definition or function call
    pub fn lookup_parameters(&self, lookup_input: LookupInputType) -> Vec<&Walker> {
        vec![]
    }

    /// Find contract based on function call return type
    pub fn lookup_contract(&self, lookup_input: LookupInputType) -> u32 {
        0
    }

    /// Find relative functions of a contract
    /// + functionDefinition of current contract
    /// + functionDefinition of its parents then parents of parents
    /// + functionDefinition of contract which is initialized in current contract
    fn lookup_contract_functions(&self, id: u32, mut contract_ids: HashSet<u32>) -> Vec<&Walker> {
        vec![]
    }

    /// Find a list of states from function_id
    /// Include inherited states
    pub fn lookup_states(&self, lookup_input: LookupInputType) -> Vec<&Walker> {
        vec![]
    }
}
