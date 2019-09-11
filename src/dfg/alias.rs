use std::collections::HashMap;
use crate::core::{
    Variable,
    VariableComparison,
    Dictionary,
    Declaration,
    Assignment,
};
use crate::cfg::ControlFlowGraph;

pub struct Alias {
    execution_tables: Vec<HashMap<u32, HashMap<Variable, Variable>>>,
}

impl Alias {
    pub fn new(cfg: &ControlFlowGraph) -> Self {
        let execution_paths = cfg.get_execution_paths();
        let dict = cfg.get_dict();
        let mut execution_tables = vec![];
        for execution_path in execution_paths {
            let mut prev_id = None;
            let mut tables: HashMap<u32, HashMap<Variable, Variable>> = HashMap::new();
            for id in execution_path {
                // Copy table from prev node
                let mut table = HashMap::new(); 
                if let Some(prev_id) = prev_id {
                    table = tables.get(prev_id).unwrap().clone();
                }
                // Find all assignments in current node
                let mut assignments = vec![];
                dict.walker_at(*id).map(|walker| {
                    assignments.extend(Assignment::parse(walker, dict));
                    for declaration in Declaration::parse(walker, dict) {
                        assignments.push(declaration.get_assignment().clone());
                    }
                });
                // Indentify whether a variable can has alias or not
                for assignment in assignments {
                    for l in assignment.get_lhs() {
                        for r in assignment.get_rhs() {
                            let aliasable = l.can_has_alias() && r.can_has_alias();
                            let same_type = l.get_type() == r.get_type();
                            let mut kill_vars = vec![];
                            if aliasable && same_type {
                                // Alias assignment is here
                                for (prev_l, _) in table.clone() {
                                    if let VariableComparison::Partial = l.contains(&prev_l) {
                                        // prev_l is child of l
                                        if prev_l.get_members().len() > l.get_members().len() {
                                            kill_vars.push(prev_l.clone());
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
                }
                prev_id = Some(id);
                tables.insert(*id, table);
            }
            execution_tables.push(tables);
        }
        Alias { execution_tables }
    }

    pub fn find_references(&self, id: u32, var: &Variable, dict: &Dictionary) -> Vec<Variable> {
        let mut references = vec![];
        for execution_table in self.execution_tables.iter() {
            if let Some(table) = execution_table.get(&id) {
                let mut stack: Vec<&Variable> = vec![var];
                let mut path_references: Vec<&Variable> = vec![];
                while stack.len() > 0 {
                    let var = stack.pop().unwrap();
                    for (left_var, right_var) in table {
                        if left_var.contains(var) == VariableComparison::Partial {
                            if left_var.get_members().len() < var.get_members().len() {
                                if !path_references.contains(&right_var) {
                                    stack.push(right_var);
                                }
                            }
                        }
                        if right_var.contains(var) == VariableComparison::Partial {
                            if right_var.get_members().len() < var.get_members().len() {
                                if !path_references.contains(&left_var) {
                                    stack.push(left_var);
                                }
                            }
                        }
                    } 
                    path_references.push(var);
                }
                path_references.remove(0);
                references.append(&mut path_references);
            }
        }
        let mut ret: Vec<Variable> = vec![];
        for reference in references {
            for r in reference.flatten(dict) {
                if r.get_members()[0] == var.get_members()[0] {
                    ret.push(r);
                }
            }
        }
        ret
    }
}
