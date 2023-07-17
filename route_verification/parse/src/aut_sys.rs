use self::lex::parse_aut_num_name;
use lazy_regex::regex_is_match;

use super::*;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AsName {
    Num(u64),
    Set(String),
    Invalid(String),
}

/// A simple AS field is either a AS number or a AS set.
/// Otherwise, return `AsExpr::Invalid`.
pub fn parse_as_name(field: String) -> Result<AsName> {
    if is_as_set(&field) {
        // AS set.
        return Ok(AsName::Set(field));
    }
    let num = parse_aut_num_name(&field).context("parsing as name")?;
    Ok(AsName::Num(num)) // AS number.
}

pub fn is_as_set(field: &str) -> bool {
    regex_is_match!(r"^(AS\d+:)?AS-\S+$"i, field)
}
