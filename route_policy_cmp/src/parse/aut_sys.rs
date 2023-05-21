use lazy_regex::regex_is_match;
use log::error;
use serde::{Deserialize, Serialize};

use super::lex::parse_aut_num_name;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AsName {
    Num(usize),
    Set(String),
    Illegal(String),
}

/// A simple AS field is either a AS number or a AS set.
/// Otherwise, return `AsExpr::Illegal`.
pub fn parse_as_name(field: String) -> AsName {
    if is_as_set(&field) {
        // AS set.
        return AsName::Set(field);
    }
    match parse_aut_num_name(&field) {
        Ok(num) => AsName::Num(num), // AS number.
        Err(err) => {
            let err = err.context("parsing as name");
            let err = format!("{err:#}");
            error!("{err}");
            AsName::Illegal(err)
        }
    }
}

pub fn is_as_set(field: &str) -> bool {
    regex_is_match!(r"^(AS\d+:)?AS-\S+$"i, field)
}
