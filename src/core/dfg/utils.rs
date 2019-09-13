pub mod Utils {
    extern crate regex;
    use crate::core::Walker;
    use crate::core::Dictionary;
    use regex::Regex;

    pub fn find_root_walker<'a>(walker: &'a Walker, dict: &'a Dictionary) -> Walker<'a> {
        if walker.direct_childs(|_| true).is_empty() || walker.node.name == "VariableDeclaration" {
            walker.clone()
        } else {
            let walker = walker.direct_childs(|_| true).get(0)
                .and_then(|walker| dict.walker_at(walker.node.id))
                .unwrap();
            find_root_walker(&walker, dict)
        }
    }

    pub fn normalize_kind(walker: &Walker) -> String {
        let type_str = walker.node.attributes["type"].as_str().unwrap_or("");
        let mut norm_type = type_str.to_string();
        for keyword in vec!["memory", "storage", "calldata", "pointer", "ref"] {
            let temp = norm_type.clone();
            norm_type.clear();
            for item in temp.split(keyword) {
                norm_type.push_str(item.trim());
            }
        }
        if type_str.starts_with("function") {
            let func_regex = Regex::new(r"returns\s*\((.+)\)$").unwrap();
            for cap in func_regex.captures_iter(&norm_type) {
                return (&cap[1]).to_string();
            }
            return "void".to_string();
        }
        return norm_type;
    } 
}
