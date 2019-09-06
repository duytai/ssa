use crate::dfg::Alias;
use std::collections::{ HashSet, HashMap };
use crate::cfg::ControlFlowGraph;
use crate::core::{
    VariableComparison,
    Operator,
    Action,
    DataLink,
    DataLinkLabel,
};
use crate::dfg::utils;

/// Data flow graph
///
/// It takes edges and vertices from the cfg to find assignments 
/// and build data flow
pub struct DataFlowGraph<'a> {
    cfg: ControlFlowGraph<'a>,
    visited: HashSet<u32>,
    parents: HashMap<u32, Vec<u32>>,
    tables: HashMap<u32, HashSet<Action>>,
    new_actions: HashMap<u32, Vec<Action>>,
    alias: Alias,
}

impl<'a> DataFlowGraph<'a> {
    /// Create new flow graph by importing `State` from cfg
    pub fn new(cfg: ControlFlowGraph<'a>, alias: Alias) -> Self {
        let vertices = cfg.get_vertices();
        let edges = cfg.get_edges();
        let mut tables = HashMap::new();
        let mut parents: HashMap<u32, Vec<u32>> = HashMap::new();
        for vertex in vertices.iter() {
            tables.insert(vertex.get_id(), HashSet::new());
        }
        for edge in edges.iter() {
            let from = edge.get_from();
            let to = edge.get_to();
            match parents.get_mut(&to) {
                Some(v) => { v.push(from); },
                None => { parents.insert(to, vec![from]); },
            }
        }
        DataFlowGraph {
            cfg,
            alias,
            parents,
            tables,
            visited: HashSet::new(),
            new_actions: HashMap::new(),
        }
    }

    pub fn get_cfg(&self) -> &ControlFlowGraph {
        &self.cfg
    }

    pub fn get_new_actions(&self) -> &HashMap<u32, Vec<Action>> {
        &self.new_actions
    }

    /// Find data dependency links
    ///
    /// Start at stop point and go bottom up. Whenever a node is visited:
    /// - If the node is a function call (Mdiamond, DoubleCircle) then we find all parameters of
    /// the function
    /// - If the node is a comparison then we find all variables in the comparison
    /// - If the node is an assignment then we find all variables in the assignment
    ///
    /// It should be noted that we ignore nested functions because each nested function takes a node in CFG.
    /// For example:
    ///
    /// ```javascript
    /// this.add(this.add(x, 1), this.add(y, 1));
    ///
    /// ```
    /// The CFG of the function call above should be: `this.add(y, 1) => this.add(x, 1) =>
    /// this.add(this.add(x, 1), this.add(y, 1))`
    /// 
    /// For each node, we build a sequence of `USING(X)` or `KILL(Y)` where X, Y are variable. For
    /// example:
    /// ```javascript
    /// uint x = y + 10; // (1)
    /// x += 20; // (2)
    /// ```
    /// (1) has the sequence: `USE(Y), KILL(X)` and (2) has the sequence: `USE(X), KILL(X)`
    ///
    /// Whenever a node is visited, we try to generate the sequence for current node and merge with
    /// the sequence of previous nodes. If the pattern `USE(X),...,KILL(X)` is discovered then
    /// all uses of variable X `USE(X)` depend on `KILL(X)`, one data dependency link is created.
    /// All elements in that pattern will be removed from the sequence.
    ///
    /// The loop will stop if no sequence changes happen
    pub fn find_links(&mut self) -> HashSet<DataLink> {
        let dict = self.cfg.get_dict();
        let stop = self.cfg.get_stop();
        let mut stack: Vec<(u32, u32, Vec<Action>)> = vec![];
        let mut links: HashSet<DataLink> = HashSet::new();
        let actions: Vec<Action> = vec![]; 
        if let Some(parents) = self.parents.get(&stop) {
            for parent in parents {
                stack.push((stop, *parent, actions.clone()));
            }
        } 
        while stack.len() > 0 {
            let (from, id, mut actions) = stack.pop().unwrap();
            let pre_table = self.tables.get(&from).unwrap().clone();
            let cur_table = self.tables.get_mut(&id).unwrap();
            let cur_table_len = cur_table.len();
            let mut new_actions = vec![];
            let mut assignments = vec![];
            let mut variables = HashSet::new();
            variables.extend(utils::find_variables(id, dict));
            assignments.append(&mut utils::find_assignments(id, dict));
            for declaration in utils::find_declarations(id, dict) {
                assignments.push(declaration.get_assignment().clone());
            }
            for function_use in utils::find_function_use(id, dict) {
                let mut agns = function_use.get_assignments().clone();
                let vars = function_use.get_variables().clone();
                assignments.append(&mut agns);
                variables.extend(vars);
            } 
            for assignment in assignments {
                for l in assignment.get_lhs().clone() {
                    match assignment.get_op() {
                        Operator::Equal => {
                            for l in l.flatten(dict) {
                                new_actions.push(Action::Kill(l, id));
                            }
                            // for l in self.alias.find_references(id, &l, dict) {
                                // new_actions.push(Action::Kill(l, id));
                            // }
                        },
                        Operator::Other => {
                            for l in l.flatten(dict) {
                                new_actions.push(Action::Kill(l.clone(), id));
                                new_actions.push(Action::Use(l, id));
                            }
                            // for l in self.alias.find_references(id, &l, dict) {
                                // new_actions.push(Action::Kill(l.clone(), id));
                                // new_actions.push(Action::Use(l, id));
                            // }
                        }
                    }
                }
                for r in assignment.get_rhs().clone() {
                    for r in r.flatten(dict) {
                        new_actions.push(Action::Use(r, id));
                    }
                    // for r in self.alias.find_references(id, &r, dict) {
                        // new_actions.push(Action::Use(r, id));
                    // }
                }
            }
            for var in variables {
                for var in var.flatten(dict) {
                    new_actions.push(Action::Use(var, id));
                }
                // for var in self.alias.find_references(id, &var, dict) {
                    // new_actions.push(Action::Use(var, id));
                // }
            }
            self.new_actions.insert(id, new_actions.clone());
            actions.extend(new_actions.clone());
            cur_table.extend(pre_table);
            cur_table.extend(new_actions);
            loop {
                let mut pos: Option<usize> = None;
                for (index, action) in actions.iter().enumerate() {
                    if let Action::Kill(_, _) = action {
                        pos = Some(index);
                        break;
                    }
                }
                if let Some(pos) = pos {
                    if let Action::Kill(kill_var, kill_id) = actions[pos].clone() {
                        actions = actions
                            .into_iter()
                            .enumerate()
                            .filter(|(index, action)| {
                                if index < &pos {
                                    if let Action::Use(variable, id) = action {
                                        match kill_var.contains(variable) {
                                            VariableComparison::Equal => {
                                                let data_link = DataLink::new(
                                                    *id,
                                                    variable.clone(),
                                                    kill_id,
                                                    kill_var.clone(),
                                                    DataLinkLabel::Internal,
                                                );
                                                links.insert(data_link);
                                                cur_table.remove(action);
                                                false
                                            },
                                            VariableComparison::Partial => {
                                                // Only kill by using parent
                                                if kill_var.get_members().len() < variable.get_members().len() {
                                                    // let data_link = DataLink::new(*id, kill_id, variable.clone());
                                                    let data_link = DataLink::new(
                                                        *id,
                                                        variable.clone(),
                                                        kill_id,
                                                        kill_var.clone(),
                                                        DataLinkLabel::Internal,
                                                    );
                                                    links.insert(data_link);
                                                    false
                                                } else {
                                                    true
                                                }
                                            },
                                            VariableComparison::NotEqual => {
                                                true
                                            },
                                        }
                                    } else {
                                        true
                                    }
                                } else if index > &pos {
                                    true
                                } else {
                                    cur_table.remove(action);
                                    false
                                }
                            })
                        .map(|(_, action)| action)
                        .collect();
                    }
                } else {
                    break;
                }
            }
            if cur_table.len() != cur_table_len || !self.visited.contains(&id) {
                self.visited.insert(id);
                if let Some(parents) = self.parents.get(&id) {
                    for parent in parents {
                        stack.push((id, *parent, actions.clone()));
                    }
                }
            }
        }
        links
    }
}
