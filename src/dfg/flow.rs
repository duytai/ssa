use std::collections::{ HashSet, HashMap };
use crate::core::{
    State,
    Shape,
    VariableComparison,
    Operator,
    Action,
    DataLink,
};
use crate::dfg::utils;

/// Data flow graph
///
/// It takes edges and vertices from the cfg to find assignments 
/// and build data flow
pub struct DataFlowGraph<'a> {
    state: &'a State<'a>,
}

impl<'a> DataFlowGraph<'a> {
    /// Create new flow graph by importing `State` from cfg
    pub fn new(state: &'a State) -> Self {
        DataFlowGraph { state }
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
    pub fn find_links(&self) -> HashSet<DataLink> {
        let State { vertices, edges, dict, stop, .. } = self.state;
        let mut visited: HashSet<u32> = HashSet::new();
        let mut stack: Vec<(u32, u32, Vec<Action>)> = vec![];
        let mut parents: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut tables: HashMap<u32, HashSet<Action>> = HashMap::new();
        let mut links: HashSet<DataLink> = HashSet::new(); 
        let actions: Vec<Action> = vec![]; 
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
        if let Some(parents) = parents.get(&stop) {
            for parent in parents {
                stack.push((*stop, *parent, actions.clone()));
            }
        } 
        while stack.len() > 0 {
            let (from, id, mut actions) = stack.pop().unwrap();
            let vertex = vertices.iter().find(|v| v.get_id() == id).unwrap();
            let pre_table = tables.get(&from).unwrap().clone();
            let cur_table = tables.get_mut(&id).unwrap();
            let cur_table_len = cur_table.len();
            let mut new_actions = vec![];
            let mut split_ats = vec![id];
            // FunctionCall => have to jump to parameters because Variable::parse ignores functionCall 
            match vertex.get_shape() {
                Shape::DoubleCircle | Shape::Mdiamond => {
                    dict.lookup(id).map(|walker| {
                        for walker in &walker.direct_childs(|_| true)[1..] {
                            split_ats.push(walker.node.id);
                        }
                    });
                },
                _ => {},
            }
            for split_at in split_ats {
                for declaration in utils::find_declarations(split_at, dict) {
                    for l in declaration.get_lhs().clone() {
                        match declaration.get_op() {
                            Operator::Equal => {
                                new_actions.push(Action::Kill(l, id));
                            },
                            Operator::Other => {
                                new_actions.push(Action::Kill(l.clone(), id));
                                new_actions.push(Action::Use(l, id));
                            }
                        }
                    }
                    for r in declaration.get_rhs().clone() {
                        new_actions.push(Action::Use(r, id));
                    }
                }
                for assignment in utils::find_assignments(split_at, dict) {
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
                for var in utils::find_variables(split_at, dict) {
                    new_actions.push(Action::Use(var, id));
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
            if cur_table.len() != cur_table_len || !visited.contains(&id) {
                visited.insert(id);
                if let Some(parents) = parents.get(&id) {
                    for parent in parents {
                        stack.push((id, *parent, actions.clone()));
                    }
                }
            }
        }
        links
    }
}
