use std::collections::{ HashSet, HashMap };
use crate::{
    vertex::{ Shape },
    dict::Dictionary,
    analyzer::{ Analyzer, State },
};
use super::{
    variable::{ Variable, VariableComparison },
    assignment::{ Assignment, Operator },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Use(Variable, u32),
    Kill(Variable, u32),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DataLink {
    from: u32,
    to: u32,
    var: Variable,
}

pub struct DataFlowGraph {}

impl DataFlowGraph {
    pub fn new() -> Self {
        DataFlowGraph {}
    }

    pub fn find_assignments(&self, id: u32, dict: &Dictionary) -> Vec<Assignment> {
        let walker = dict.lookup(id).unwrap();
        Assignment::parse(walker, dict)
    }

    pub fn find_variables(&self, id: u32, dict: &Dictionary) -> HashSet<Variable> {
        let walker = dict.lookup(id).unwrap();
        Variable::parse(walker, dict)
    }

    pub fn find_parameters(&self, id: u32, dict: &Dictionary) -> HashSet<Variable> {
        let walker = dict.lookup(id).unwrap();
        let mut variables = HashSet::new();
        walker.for_all(|_| { true }, |walkers| {
            for walker in &walkers[1..] {
                let vars = Variable::parse(walker, dict);
                variables.extend(vars);
            }
        });
        variables
    }
}

impl Analyzer for DataFlowGraph {
    fn analyze(&mut self, state: &mut State) {
        let stop = 1000000;
        let State { vertices, edges, dict, .. } = state;
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
                stack.push((stop, *parent, actions.clone()));
            }
        } 
        while stack.len() > 0 {
            let (from, id, mut actions) = stack.pop().unwrap();
            let vertex = vertices.iter().find(|v| v.id == id).unwrap();
            let pre_table = tables.get(&from).unwrap().clone();
            let cur_table = tables.get_mut(&id).unwrap();
            let cur_table_len = cur_table.len();
            let mut new_actions = vec![];
            let mut kill_pos = vec![];
            match vertex.shape {
                Shape::DoubleCircle => {
                    for var in self.find_parameters(id, dict) {
                        new_actions.push(Action::Use(var, id));
                    }
                },
                Shape::Box => {
                    let assignments = self.find_assignments(id, dict);
                    if assignments.len() > 0 {
                        for assignment in assignments {
                            let Assignment { lhs, rhs, op } = assignment;
                            for l in lhs {
                                match op {
                                    Operator::Equal => {
                                        kill_pos.push(actions.len() + new_actions.len());
                                        new_actions.push(Action::Kill(l, id));
                                    },
                                    Operator::Other => {
                                        kill_pos.push(actions.len() + new_actions.len());
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
                        for var in self.find_variables(id, dict) {
                            new_actions.push(Action::Use(var, id));
                        }
                    }
                },
                Shape::Diamond => {
                    for var in self.find_variables(id, dict) {
                        new_actions.push(Action::Use(var, id));
                    }
                },
                Shape::Point => {},
            }
            actions.extend(new_actions.clone());
            cur_table.extend(pre_table);
            cur_table.extend(new_actions);
            for pos in kill_pos {
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
        state.links = Some(links);
    }
}
