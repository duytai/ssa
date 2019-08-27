use crate::dfg::Network;
use crate::core::{
    Walker,
    Variable,
    Dictionary,
};

/// Detect overflow:
///
/// Find these expression: 
/// + Result = Expression0 + Expression1
/// + Result += Expression0
///
/// Overflow if: type(Result) <= type(Expression0) || type(Result) <= type(Expression1)
pub struct IntegerOverflow {}

impl IntegerOverflow {

    fn get_operand_type_size(variable_type: &str) -> Option<u32> {
        let mut ret = None;
        if variable_type.starts_with("int_const") {
            // Constant
            ret = Some(256);
        } else if variable_type.starts_with("uint") {
            // Unsigned int
            ret = variable_type[4..].parse::<u32>().ok();
        } else if variable_type.starts_with("int") {
            // Signed int
            ret = variable_type[3..].parse::<u32>().ok();
        }
        ret
    }

    fn get_operand_type(walker: &Walker, dict: &Dictionary) -> Option<String> {
        match Variable::parse_one(&walker, dict) {
            Some (variable) => variable.get_type().clone().map(|x| x.to_string()),
            None => Variable::normalize_type(&walker),
        }
    } 

    pub fn analyze<'a>(network: &'a Network<'a>) -> Vec<(Walker, String)> {
        let mut expressions = vec![];
        // Dictionary that we can lookup 
        let dict = network.get_dict();
        // List of data flow graph, one dfg for a function 
        let dfgs = network.get_dfgs();
        // id is id of FunctionDefinition node 
        for (id, _) in dfgs {
            if let Some(walker) = dict.lookup(*id) {
                // Search binary operator + / * / += / *=
                let filter = |walker: &Walker, _: &Vec<Walker>| {
                    let operator = walker.node.attributes["operator"].as_str().unwrap_or("");
                    (walker.node.name == "BinaryOperation" && (operator == "+" || operator == "*")) ||
                    (walker.node.name == "Assignment" && (operator == "+=" || operator == "*=")) ||
                    (walker.node.name == "UnaryOperation" && operator == "++")
                };
                let ignore = |_: &Walker, _: &Vec<Walker>| false;
                let walkers = walker.walk(false, ignore, filter);
                for walker in walkers {
                    let operator = walker.node.attributes["operator"].as_str().unwrap_or("");
                    match operator {
                        "++" => {
                            let reason = String::from("used operator ++");
                            expressions.push((walker, reason));
                        },
                        _ => {
                            let return_type = Variable::normalize_type(&walker);
                            let walkers = walker.direct_childs(|_| true);
                            let left_type = IntegerOverflow::get_operand_type(&walkers[0], dict); 
                            let right_type = IntegerOverflow::get_operand_type(&walkers[1], dict);
                            if let (Some(left_type), Some(right_type), Some(return_type)) = (left_type, right_type, return_type) {
                                let left_size = IntegerOverflow::get_operand_type_size(&left_type);
                                let right_size = IntegerOverflow::get_operand_type_size(&right_type);
                                let return_size = IntegerOverflow::get_operand_type_size(&return_type);
                                if let (Some(left_size), Some(right_size), Some(return_size)) = (left_size, right_size, return_size) {
                                    if operator == "+" || operator == "+=" {
                                        if return_size <= left_size {
                                            let reason = String::from("return size <= left size");
                                            expressions.push((walker, reason));
                                        } else if return_size <= right_size {
                                            let reason = String::from("return size <= right size");
                                            expressions.push((walker, reason));
                                        }
                                    } else if  operator == "*" || operator == "*=" {
                                        if return_size < (left_size + right_size) {
                                            let reason = String::from("return size < right size + left size");
                                            expressions.push((walker, reason));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        expressions
    }
}

