use serde::{Deserialize, Serialize};

use crate::lex::filter;

pub fn parse_filter(mp_filter: filter::Filter) -> Filter {
    todo!()
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
// TODO: Fill in.
pub enum Filter {}
