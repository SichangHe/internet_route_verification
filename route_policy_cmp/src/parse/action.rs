use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub type Actions = BTreeMap<String, Action>;

// TODO: Fill in.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum Action {}
