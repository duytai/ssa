/// Variable access
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Member {
    /// Link to node that defines current variable
    Reference(u32),
    /// Contains name of a global variable or function
    Global(String),
    /// Accesses a member in an array
    IndexAccess,
}


