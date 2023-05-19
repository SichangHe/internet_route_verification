use lazy_regex::regex_captures;
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
pub fn parse_as_name(field: &str) -> AsName {
    if let Some(name) = try_parse_as_set(field) {
        // AS set.
        return AsName::Set(name.into());
    }
    match parse_aut_num_name(field) {
        Ok(num) => AsName::Num(num), // AS number.
        Err(err) => {
            let err = format!("{err:?}");
            error!("{err}");
            AsName::Illegal(err)
        }
    }
}

pub fn try_parse_as_set(field: &str) -> Option<&str> {
    regex_captures!(r"^AS-(\S+)$"i, field).map(|(_, name)| name)
}
