use serde::{Deserialize, Serialize};

use crate::lex::filter;

pub fn parse_filter(mp_filter: filter::Filter) -> Filter {
    Filter::Todo
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum Filter {
    Todo, // TODO: Fill in.
}
