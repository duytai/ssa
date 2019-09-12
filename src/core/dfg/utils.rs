pub mod Utils {
    use crate::core::Walker;
    use crate::core::Dictionary;

    pub fn find_root_walker<'a>(walker: &'a Walker, dict: &'a Dictionary) -> Walker<'a> {
        if walker.direct_childs(|_| true).is_empty() {
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
        norm_type
    } 
}
