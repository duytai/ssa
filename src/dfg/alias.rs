use crate::cfg::ControlFlowGraph;
use crate::dfg::utils;
/// Detect aliasing in program
/// A = B
/// A is a struct instance => Same for B
/// A is a contract instance => Same for B
/// A is a array instance => Same for B
/// Solidty does not allow to has function in B
pub struct Alias<'a> {
    cfg: &'a ControlFlowGraph<'a>,
}

impl<'a> Alias<'a> {
    pub fn new(cfg: &'a ControlFlowGraph<'a>) -> Self {
        let alias = Alias { cfg };
        alias.find_assignment();
        alias
    }

    fn find_assignment(&self) {
        let dict = self.cfg.get_dict();
        let vertices = self.cfg.get_vertices();
        for vertext in vertices {
            let id = vertext.get_id();
            let mut assignments = vec![];
            assignments.append(&mut utils::find_assignments(id, dict));
            for declaration in utils::find_declarations(id, dict) {
                assignments.push(declaration.get_assignment().clone());
            }
            for function_use in utils::find_function_use(id, dict) {
                let mut agns = function_use.get_assignments().clone();
                assignments.append(&mut agns);
            } 
            // for assignment in assignments {
                // for l in assignment.get_lhs() {
                    // println!("L: {:?}", l);
                // }
                // for r in assignment.get_rhs() {
                    // println!("R: {:?}", r);
                // }
            // }
        }
    }
}
