use std::collections::HashMap;
use crate::core::{
    Variable,
    VariableComparison,
};
use crate::cfg::ControlFlowGraph;
use crate::dfg::utils;

type AliasTable = HashMap<Variable, Variable>;
type AliasTables = HashMap<u32, AliasTable>;

pub struct Alias {
    execution_tables: Vec<AliasTables>,
}

impl Alias {
    pub fn new(cfg: &ControlFlowGraph) -> Self {
        let execution_paths = cfg.get_execution_paths();
        let dict = cfg.get_dict();
        let mut execution_tables = vec![];
        for execution_path in execution_paths {
            let mut prev_id = None;
            let mut tables: AliasTables = HashMap::new();
            for id in execution_path {
                // Copy table from prev node
                let mut table: AliasTable = HashMap::new(); 
                if let Some(prev_id) = prev_id {
                    table = tables.get(prev_id).unwrap().clone();
                }
                // Find all assignments in current node
                let mut assignments = vec![];
                assignments.append(&mut utils::find_assignments(*id, dict));
                for declaration in utils::find_declarations(*id, dict) {
                    assignments.push(declaration.get_assignment().clone());
                }
                // Indentify whether a variable can has alias or not
                for assignment in assignments {
                    for l in assignment.get_lhs() {
                        for r in assignment.get_rhs() {
                            let alias_able = l.can_has_alias() && r.can_has_alias();
                            let same_type = l.get_type() == r.get_type();
                            let mut kill_vars = vec![];
                            if alias_able && same_type {
                                // Alias assignment is here
                                for (prev_l, _) in table.clone() {
                                    if let VariableComparison::Partial = l.contains(&prev_l) {
                                        // prev_l is child of l
                                        if prev_l.get_members().len() > l.get_members().len() {
                                            kill_vars.push(prev_l.clone());
                                        }
                                    }
                                }
                            }
                            // Delete all childs 
                            for kill_var in kill_vars.iter() {
                                table.remove(kill_var);
                            }
                            // Insert l_var to current table
                            table.insert(l.clone(), r.clone());
                        }
                    }
                }
                prev_id = Some(id);
                tables.insert(*id, table);
            }
            execution_tables.push(tables);
        }
        Alias { execution_tables }
    }

    pub fn find_references(&self, id: u32, var: &Variable) {
        println!("find-reference for: {:?} at {}", var, id);
        for execution_table in self.execution_tables.iter() {
            for (id, table) in execution_table {
                for (l_var, r_var) in table {
                    println!("id: {}", id);
                    println!("v: {}", id);
                }
            }
        }
    }
}
