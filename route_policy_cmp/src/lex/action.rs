use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::community::Call;

pub type Actions = BTreeMap<String, Action>;

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum Action {
    Assigned(String),
    AssignedSet(Vec<String>),
    MethodCall(Vec<Call>),
}
