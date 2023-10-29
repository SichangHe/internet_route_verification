use self::lex::{is_pseudo_set, parse_aut_num_name};
use super::*;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AsName {
    Any,
    Num(u32),
    Set(String),
    Invalid(String),
}

/// A simple AS field is either a AS number or a AS set.
/// Otherwise, return `AsExpr::Invalid`.
pub fn parse_as_name(field: String) -> Result<AsName> {
    Ok(if is_any(&field) {
        AsName::Any
    } else if is_as_set(&field) || is_pseudo_set(&field) {
        AsName::Set(field) // AS set.
    } else {
        let num = parse_aut_num_name(&field).context("parsing as name")?;
        AsName::Num(num) // AS number.
    })
}

pub fn is_as_set(field: &str) -> bool {
    regex!(formatcp!("^{}$", AS_SET)).is_match(field)
}
