use std::collections::HashSet;
use std::cmp;
use crate::core::{
    Walker,
    Dictionary,
    Member,
    VariableComparison,
    FlatVariable,
    DataLink,
    DataLinkLabel,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Variable {
    members: Vec<Member>,
    source: String,
    kind: String,
}

impl Variable {

    pub fn new(members: Vec<Member>, source: String, kind: String) -> Self {
        Variable { members, source, kind }
    }

    pub fn get_members(&self) -> &Vec<Member> {
        &self.members
    }

    pub fn get_source(&self) -> &str {
        &self.source
    }

    pub fn get_kind(&self) -> &str {
        &self.kind
    }

    pub fn parse(walker: &Walker, dict: &Dictionary) -> HashSet<Self> {
        let mut ret = HashSet::new();
        let fi = |walker: &Walker, _: &Vec<Walker>| {
            walker.node.name == "MemberAccess"
            || walker.node.name == "Identifier"
            || walker.node.name == "IndexAccess"
            || walker.node.name == "FunctionCall"
        };
        let ig = |walker: &Walker, _: &Vec<Walker>| {
            walker.node.name == "VariableDeclaration"
            || walker.node.name == "VariableDeclarationStatement"
            || walker.node.name == "Assignment"
        };
        for walker in walker.walk(true, ig, fi) {
            let flat_variable = FlatVariable::new(&walker, dict);
            ret.extend(flat_variable.get_variables());
        }
        ret
    }

    pub fn equal_property(&self, other: &Variable) -> bool {
        if self.get_kind() == other.get_kind() {
            let mut members = self.members.clone();
            let mut other_members = other.members.clone();
            members.reverse();
            other_members.reverse();
            let mut idx = 0;
            while idx < cmp::min(members.len(), other_members.len()) {
                let member = &members[idx];
                let other_member = &other_members[idx];
                match member == other_member {
                    true => match member == &Member::IndexAccess {
                        true => idx += 1,
                        false => return true,
                    },
                    false => return false,
                }
            }
        }
        false
    }


    pub fn contains(&self, other: &Variable) -> VariableComparison {
        if other.members.len() > self.members.len() {
            let sub = &other.members[..self.members.len()];
            match sub.iter().eq(self.members.iter()) {
                true => VariableComparison::Partial,
                false => VariableComparison::NotEqual,
            }
        } else {
            let sub = &self.members[..other.members.len()];
            match sub.iter().eq(other.members.iter()) {
                true => {
                    match other.members.len() == self.members.len() {
                        true => VariableComparison::Equal,
                        false => VariableComparison::Partial,
                    }
                },
                false => VariableComparison::NotEqual,
            }
        }
    }

    pub fn links(kill_variables_tup: (HashSet<Variable>, u32), use_variables_tup: (HashSet<Variable>, u32), label: DataLinkLabel) -> HashSet<DataLink> {
        let mut assignment_links = HashSet::new();
        let (kill_variables, kill_id) = kill_variables_tup;
        let (use_variables, use_id) = use_variables_tup;
        match label {
            DataLinkLabel::SameType => {
                if kill_variables.len() == 1 {
                    for kill_variable in kill_variables.iter() {
                        for use_variable in use_variables.iter() {
                            if kill_variable.get_kind() == use_variable.get_kind() {
                                let data_link = DataLink::new(
                                    (kill_variable.clone(), kill_id),
                                    (use_variable.clone(), use_id),
                                    DataLinkLabel::SameType,
                                );
                                assignment_links.insert(data_link);
                            }
                        }
                    }
                }
                if kill_variables.len() > 1 {
                    for kill_variable in kill_variables.iter() {
                        for use_variable in use_variables.iter() {
                            if kill_variable.equal_property(use_variable) {
                                let data_link = DataLink::new(
                                    (kill_variable.clone(), kill_id),
                                    (use_variable.clone(), use_id),
                                    DataLinkLabel::SameType,
                                );
                                assignment_links.insert(data_link);
                            }
                        }
                    }
                }
            },
            DataLinkLabel::SwitchType => {
                for kill_variable in kill_variables.iter() {
                    for use_variable in use_variables.iter() {
                        let data_link = DataLink::new(
                            (kill_variable.clone(), kill_id),
                            (use_variable.clone(), use_id),
                            DataLinkLabel::SwitchType,
                        );
                        assignment_links.insert(data_link);
                    }
                }
            },
        }
        assignment_links
    }
}
