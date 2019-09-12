pub mod Utils {
    use crate::core::Walker;
    use crate::core::Dictionary;

    pub fn find_root_kind(walker: &Walker, dict: &Dictionary) -> String {
        if walker.direct_childs(|_| true).is_empty() {
            normalize_kind(walker)
        } else {
            walker.direct_childs(|_| true).get(0)
                .map(|walker| find_root_kind(&walker, dict))
                .unwrap_or(String::from(""))
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
        norm_type
    } 
}
