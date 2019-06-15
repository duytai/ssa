use std::collections::{ HashSet, HashMap };
use crate::core::{ Shape, State };
use crate::{
    variable::{ VariableComparison },
    assignment::{ Assignment, Operator },
};
use crate::action::Action;
use crate::link::DataLink;
use crate::utils;

pub struct DataFlowGraph<'a> {
    state: &'a State<'a>,
}

impl<'a> DataFlowGraph<'a> {
    pub fn new(state: &'a State) -> Self {
        DataFlowGraph { state }
    }

    pub fn find_links(&self) -> HashSet<DataLink> {
        let State { vertices, edges, dict, stop, .. } = self.state;
        let mut visited: HashSet<u32> = HashSet::new();
        let mut stack: Vec<(u32, u32, Vec<Action>)> = vec![];
        let mut parents: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut tables: HashMap<u32, HashSet<Action>> = HashMap::new();
        let mut links: HashSet<DataLink> = HashSet::new(); 
        let actions: Vec<Action> = vec![]; 
        for vertex in vertices.iter() {
            tables.insert(vertex.id, HashSet::new());
        }
        for (from, to) in edges.iter() {
            match parents.get_mut(to) {
                Some(v) => { v.push(*from); },
                None => { parents.insert(*to, vec![*from]); },
            }
        }
        if let Some(parents) = parents.get(&stop) {
            for parent in parents {
                stack.push((*stop, *parent, actions.clone()));
            }
        } 
        while stack.len() > 0 {
            let (from, id, mut actions) = stack.pop().unwrap();
            let vertex = vertices.iter().find(|v| v.id == id).unwrap();
            let pre_table = tables.get(&from).unwrap().clone();
            let cur_table = tables.get_mut(&id).unwrap();
            let cur_table_len = cur_table.len();
            let mut new_actions = vec![];
            match vertex.shape {
                Shape::DoubleCircle | Shape::Mdiamond => {
                    for var in utils::find_parameters(id, dict) {
                        new_actions.push(Action::Use(var, id));
                    }
                },
                Shape::Box => {
                    let assignments = utils::find_assignments(id, dict);
                    if assignments.len() > 0 {
                        for assignment in assignments {
                            let Assignment { lhs, rhs, op } = assignment;
                            for l in lhs {
                                match op {
                                    Operator::Equal => {
                                        new_actions.push(Action::Kill(l, id));
                                    },
                                    Operator::Other => {
                                        new_actions.push(Action::Kill(l.clone(), id));
                                        new_actions.push(Action::Use(l, id));
                                    }
                                }
                            }
                            for r in rhs {
                                new_actions.push(Action::Use(r, id));
                            }
                        }
                    } else {
                        for var in utils::find_variables(id, dict) {
                            new_actions.push(Action::Use(var, id));
                        }
                    }
                },
                Shape::Diamond => {
                    for var in utils::find_variables(id, dict) {
                        new_actions.push(Action::Use(var, id));
                    }
                },
                Shape::Point => {},
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
                                                let data_link = DataLink {
                                                    from: *id,
                                                    to: kill_id,
                                                    var: variable.clone(),
                                                };
                                                links.insert(data_link);
                                                cur_table.remove(action);
                                                false
                                            },
                                            VariableComparison::Partial => {
                                                if kill_var.members.len() > variable.members.len() {
                                                    let data_link = DataLink {
                                                        from: *id,
                                                        to: kill_id,
                                                        var: kill_var.clone(),
                                                    };
                                                    links.insert(data_link);
                                                } else {
                                                    let data_link = DataLink {
                                                        from: *id,
                                                        to: kill_id,
                                                        var: variable.clone(),
                                                    };
                                                    links.insert(data_link);
                                                }
                                                cur_table.remove(action);
                                                true
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
