use crate::cfg::SimpleBlockNode;
use crate::core::Walker;

pub struct Splitter {}

impl Splitter {
    pub fn new() -> Self {
        Splitter {}
    }

    pub fn split<'a>(&self, walker: Walker<'a>) -> Vec<SimpleBlockNode<'a>> {
        let mut function_calls = vec![];
        let ig = |_: &Walker, _: &Vec<Walker>| false;
        let fi = |walker: &Walker, _: &Vec<Walker>| {
            walker.node.name == "FunctionCall"
            || walker.node.name == "ModifierInvocation"
            || walker.node.name == "IndexAccess"
        };
        // Split parameters to other nodes
        for walker in walker.walk(true, ig, fi).into_iter() {
            for walker in walker.direct_childs(|_| true).into_iter() {
                function_calls.append(&mut self.split(walker));
            }
            match walker.node.name {
                "FunctionCall" => {
                    walker.direct_childs(|_| true).get(0)
                        .and_then(|walker| {
                            let function_name = walker.node.attributes["value"].as_str();
                            let member_name = walker.node.attributes["member_name"].as_str();
                            let reference = walker.node.attributes["referencedDeclaration"].as_u32();
                            function_name.or(member_name).and_then(|name| Some((name, reference)))
                        })
                        .map(|identity| match identity {
                            ("revert", _) => {
                                let node = SimpleBlockNode::Revert(walker);
                                function_calls.push(node);
                            },
                            ("assert", _) => {
                                let node = SimpleBlockNode::Assert(walker);
                                function_calls.push(node);
                            },
                            ("require", _) => {
                                let node = SimpleBlockNode::Require(walker);
                                function_calls.push(node);
                            },
                            ("suicide", _) => {
                                let node = SimpleBlockNode::Suicide(walker);
                                function_calls.push(node);
                            },
                            ("selfdestruct", _) => {
                                let node = SimpleBlockNode::Selfdestruct(walker);
                                function_calls.push(node);
                            },
                            ("transfer", None) => {
                                let node = SimpleBlockNode::Transfer(walker);
                                function_calls.push(node);
                            },
                            _ => {
                                let node = SimpleBlockNode::FunctionCall(walker);
                                function_calls.push(node);
                            }
                        });
                },
                "ModifierInvocation" => {
                    let node = SimpleBlockNode::ModifierInvocation(walker);
                    function_calls.push(node);
                },
                "IndexAccess" => {
                    let node = SimpleBlockNode::IndexAccess(walker);
                    function_calls.push(node);
                },
                _ => {},
            }
        }
        if !vec![
            "FunctionCall",
            "ModifierInvocation",
            "IndexAccess"
        ].contains(&walker.node.name) {
            let node = SimpleBlockNode::Unit(walker.clone());
            function_calls.push(node);
        }
        function_calls
    }

}
