use std::collections::HashSet;
use crate::core::{
    Assignment,
    Variable,
    Dictionary,
    Walker,
    Member,
    Operator,
};

/// Fake node
///
/// We use fake node to analyze function call, from original source code: 
/// ```javascript
/// this.add(10, this.add(x, this.add(y, 0)))
/// ```
/// We convert it to
/// ```javascript
/// fake_100 = this.add(y, 0);
/// fake_101 = this.add(x, fake_100);
/// fake_102 = this.add(10, fake_101);
/// ```
/// where {100, 101, 102} are id of functioncall, it guarantees that each fake variable is uniqe
/// It is noted that fake variable is actually global variable
pub struct FakeNode {
    assignments: Vec<Assignment>,
    variables: HashSet<Variable>,
}

impl FakeNode {
    pub fn get_assignments(&self) -> &Vec<Assignment> {
        &self.assignments
    }

    pub fn get_variables(&self) -> &HashSet<Variable> {
        &self.variables
    }

    pub fn parse(walker: &Walker, _: &Dictionary) -> Vec<FakeNode> {
        let mut ret = vec![];
        let ig = |_: &Walker, _: &Vec<Walker>| false;
        let fi = |walker: &Walker, _: &Vec<Walker>| {
            walker.node.name == "FunctionCall"
        };
        let mut candidate_walkers = vec![];
        if walker.node.name == "FunctionCall" {
            for walker in walker.direct_childs(|_| true) {
                candidate_walkers.append(&mut walker.walk(true, ig, fi));
            }
            ret.push(FakeNode::parse_one(walker, true));
        } else {
            candidate_walkers.append(&mut walker.walk(true, ig, fi));
        }
        for walker in candidate_walkers {
            ret.push(FakeNode::parse_one(&walker, false));
        }
        ret
    }

    pub fn parse_one(walker: &Walker, is_assignment: bool) -> FakeNode {
        let name = format!("fake_{}", walker.node.id);
        let source = format!("{} = {}", name, walker.node.source);
        let variable = Variable::new(
            vec![Member::Global(name)],
            source,
        );
        match is_assignment {
            false => {
                let mut variables = HashSet::new();
                variables.insert(variable);
                FakeNode {
                    variables,
                    assignments: vec![],
                }
            },
            true => {
                let mut lhs = HashSet::new();
                let rhs = HashSet::new();
                let op = Operator::Equal;
                lhs.insert(variable);
                let assignment = Assignment::new(lhs, rhs, op);
                FakeNode {
                    variables: HashSet::new(),
                    assignments: vec![assignment],
                }
            }
        }
    }
}
