use crate::dfg::Network;
use std::collections::HashSet;
use std::collections::HashMap;
use crate::core::{
    Action,
    Member,
    Shape,
};

/// How to check:
/// parameters of suicide/selfdestruct depend on parameters or msg.sender
/// but there is condition check against parameters or msg.sender 
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

        let is_valid_condition = |sending_at: u32, condition_at: u32| -> bool {
            let mut sending_level = 0;
            let mut condition_level = 0;
            for vertice in all_vertices.iter() {
                let vertex_id = vertice.get_id();
                let shape = vertice.get_shape();
                let level = vertice.get_level();
                if vertex_id == condition_at && shape == &Shape::Diamond {
                    condition_level = level;
                }
                if vertex_id == sending_at {
                    sending_level = level;
                }
            }
            sending_level * condition_level > 0 && sending_level > condition_level 
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
                            // Find suicide/selfdestruct
                            if variable_members == &suicide_members || variable_members == &selfdestruct_members {
                                let source = (variable.clone(), vertex_id);
                                // Depend on parameters or not
                                for dependent_path in network.traverse(source) {
                                    if dependent_path.len() > 1 {
                                        let (variable, _) = dependent_path.last().unwrap();
                                        // Depend on msg.sender 
                                        if variable.get_source().starts_with("msg.sender") {
                                            // If msg.sender involves in condition check 
                                            for i in 0..idx {
                                                if is_valid_condition(vertex_id, execution_path[i]) {
                                                    println!("condition: {}", execution_path[i]);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                idx -= 1;
            }
        }
    }
}


