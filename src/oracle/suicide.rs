use crate::dfg::Network;
use std::collections::HashSet;
use std::collections::HashMap;
use crate::core::{
    Action,
    Member,
    Shape,
};

pub struct Suicide {
}

impl Suicide {
    pub fn new(network: &Network) -> Suicide {
        let mut suicide = Suicide {};
        suicide.update(network);
        suicide
    }

    fn update(&mut self, network: &Network) {
        let mut all_actions = HashMap::new();
        let mut all_edges = HashSet::new();
        let mut execution_paths = vec![];
        let mut all_vertices = HashSet::new();
        let dict = network.get_dict();

        for (_, dfg) in network.get_dfgs().iter() {
            let cfg = dfg.get_cfg();
            all_actions.extend(dfg.get_new_actions());
            all_edges.extend(cfg.get_edges());
            all_vertices.extend(cfg.get_vertices());
            execution_paths.extend(cfg.get_execution_paths());
        }

        let is_condition = |id: u32| -> bool {
            for vertice in all_vertices.iter() {
                if id == vertice.get_id() {
                    return vertice.get_shape() == &Shape::Diamond;
                }
            }
            false
        };

        let get_variables = |id: u32| {
            let mut variables = HashSet::new();
            if let Some(actions) = all_actions.get(&id) {
                for action in actions.iter() {
                    match action {
                        Action::Use(variable, _) => {
                            variables.insert(variable.clone());
                        },
                        Action::Kill(variable, _) => {
                            variables.insert(variable.clone());
                        },
                    }
                }
            }
            variables
        };

        let mut possible_vul_vertices: HashSet<u32> = HashSet::new();
        for execution_path in execution_paths {
            let mut idx = execution_path.len() - 1;
            while idx > 0 {
                let vertex_id = execution_path[idx];
                if let Some(walker) = dict.walker_at(vertex_id) {
                    if walker.node.name == "FunctionCall" {
                        for variable in get_variables(vertex_id) {
                            let variable_members = variable.get_members();
                            let suicide_members = vec![Member::Global(String::from("suicide"))];
                            let selfdestruct_members = vec![Member::Global(String::from("selfdestruct"))];
                            if variable_members == &suicide_members || variable_members == &selfdestruct_members {
                                let mut has_condition = false;
                                for i in 0..idx {
                                    has_condition = has_condition || is_condition(execution_path[i]);
                                }
                                if !has_condition {
                                    possible_vul_vertices.insert(vertex_id);
                                }
                                idx = 1;
                            }
                        }
                    }
                }
                idx -= 1;
            }
        }

        println!("vul_vertices: {:?}", possible_vul_vertices);
    }
}


