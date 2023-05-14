use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::lex::action;

pub type Actions = BTreeMap<String, Action>;

pub fn parse_actions(actions: action::Actions) -> Actions {
    let parsed = BTreeMap::new();
    // TODO: Implement.
    parsed
}

// TODO: Fill in.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum Action {}
