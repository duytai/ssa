use std::collections::{ HashSet, HashMap };
use crate::cfg::ControlFlowGraph;
use crate::core::{
    VariableComparison,
    Operator,
    Action,
    DataLink,
    Variable,
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
    opens: HashSet<u32>,
}

pub type DataFlowReturnContext = Option<(u32, HashSet<Variable>)>;
pub type DataFlowParamContext = Option<(u32, Vec<HashSet<Variable>>)>;

impl<'a> DataFlowGraph<'a> {
    /// Create new flow graph by importing `State` from cfg
    pub fn new(cfg: ControlFlowGraph<'a>) -> Self {
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
            parents,
            tables,
            visited: HashSet::new(),
            opens: HashSet::new(),
        }
    }

    pub fn get_opens(&self) -> &HashSet<u32> {
        &self.opens
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
    pub fn find_links(&mut self, ctx_params: DataFlowParamContext, ctx_returns: DataFlowReturnContext) -> HashSet<DataLink> {
        let dict = self.cfg.get_dict();
        let stop = self.cfg.get_stop();
        let start = self.cfg.get_start();
        let mut stack: Vec<(u32, u32, Vec<Action>)> = vec![];
        let mut links: HashSet<DataLink> = HashSet::new();
        let mut actions: Vec<Action> = vec![]; 
        if let Some(ctx_returns) = &ctx_returns {
            for var in ctx_returns.1.iter() {
                actions.push(Action::Use(var.clone(), ctx_returns.0));
                actions.push(Action::Kill(var.clone(), stop));
                actions.push(Action::Use(var.clone(), stop));
            } 
        }
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
            for index_access in utils::find_index_accesses(id, dict) {
                let mut agns = index_access.get_assignments().clone();
                let vars = index_access.get_variables().clone();
                assignments.append(&mut agns);
                variables.extend(vars);
            }
            for parameter in utils::find_parameters(id, dict) {
                let mut agns = parameter.get_assignments().clone();
                let vars = parameter.get_variables().clone();
                assignments.append(&mut agns);
                variables.extend(vars);
            }
            for fake_node in utils::find_fake_nodes(id, dict) {
                let mut agns = fake_node.get_assignments().clone();
                let vars = fake_node.get_variables().clone();
                assignments.append(&mut agns);
                variables.extend(vars);
            }
            for assignment in assignments {
                for l in assignment.get_lhs().clone() {
                    match assignment.get_op() {
                        Operator::Equal => {
                            new_actions.push(Action::Kill(l, id));
                        },
                        Operator::Other => {
                            new_actions.push(Action::Kill(l.clone(), id));
                            new_actions.push(Action::Use(l, id));
                        }
                    }
                }
                for r in assignment.get_rhs().clone() {
                    new_actions.push(Action::Use(r, id));
                }
            }
            for var in variables {
                new_actions.push(Action::Use(var, id));
            }
            if let Some(walker) = dict.lookup(id) {
                match walker.node.name {
                    "FunctionCall" => {
                        self.opens.insert(id);
                    },
                    "Return" => {
                        // Link from stop to Return statement 
                        if let Some(ctx_returns) = &ctx_returns {
                            for var in ctx_returns.1.iter() {
                                new_actions.push(Action::Kill(var.clone(), id));
                                // TODO: Add this line to make sure variables bubble up 
                                new_actions.push(Action::Use(var.clone(), id));
                            } 
                        }
                    },
                    "ParameterList" => {
                        // Link from parameters to start
                        if let Some(ctx_params) = &ctx_params {
                            for vars in ctx_params.1.iter() {
                                for var in vars {
                                    new_actions.push(Action::Use(var.clone(), id));
                                }
                            } 
                        }
                    },
                    _ => {},
                }
            }
            if id == start {
                if let Some(ctx_params) = &ctx_params {
                    for vars in ctx_params.1.iter() {
                        for var in vars {
                            new_actions.push(Action::Kill(var.clone(), id));
                            new_actions.push(Action::Use(var.clone(), id));
                            new_actions.push(Action::Kill(var.clone(), ctx_params.0));
                        }
                    } 
                }
            }
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
                                                let data_link = DataLink::new(*id, kill_id, variable.clone());
                                                links.insert(data_link);
                                                cur_table.remove(action);
                                                false
                                            },
                                            VariableComparison::Partial => {
                                                if kill_var.get_members().len() > variable.get_members().len() {
                                                    let data_link = DataLink::new(*id, kill_id, kill_var.clone());
                                                    links.insert(data_link);
                                                } else {
                                                    let data_link = DataLink::new(*id, kill_id, variable.clone());
                                                    links.insert(data_link);
                                                }
                                                cur_table.remove(action);
                                                false
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
            println!("condition {} != {}", cur_table.len(), cur_table_len);
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
