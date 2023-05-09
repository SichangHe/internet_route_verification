use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub type Actions = BTreeMap<String, Action>;

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum Action {
    Assigned(String),
    AssignedSet(Vec<String>),
    MethodCall(Vec<Call>),
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Call {
    pub method: Option<String>,
    pub args: Vec<String>,
}
