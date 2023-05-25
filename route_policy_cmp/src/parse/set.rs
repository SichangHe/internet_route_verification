use anyhow::{Context, Result};
use lazy_regex::{regex_captures, regex_is_match};
use serde::{Deserialize, Serialize};

use super::{
    address_prefix::{AddrPfxRange, RangeOperator},
    aut_sys::AsName,
    peering::Peering,
};

/// <https://www.rfc-editor.org/rfc/rfc2622#section-5.1>
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AsSet {
    pub body: String,
    pub members: Vec<AsName>,
}

pub fn is_route_set_name(attr: &str) -> bool {
    regex_is_match!(r"^(AS\d+:)?rs-\S+$"i, attr)
}

/// <https://www.rfc-editor.org/rfc/rfc2622#section-5.2>
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RouteSet {
    pub body: String,
    /// List of `<address-prefix-range>` or `<route-set-name>` or
    /// `<route-set-name><range-operator>`.
    pub members: Vec<RouteSetMember>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RouteSetMember {
    /// `<address-prefix-range>`
    Range(AddrPfxRange),
    /// `<route-set-name>`
    Name(String),
    /// `<route-set-name><range-operator>`
    NameOp(String, RangeOperator),
}

impl From<String> for RouteSetMember {
    fn from(value: String) -> Self {
        if let Ok(range) = value.parse() {
            Self::Range(range)
        } else if let Ok((name, op)) = try_parse_name_operator(&value) {
            Self::NameOp(name.into(), op)
        } else {
            Self::Name(value)
        }
    }
}

pub fn try_parse_name_operator(s: &str) -> Result<(&str, RangeOperator)> {
    let (_, name, operator) = get_name_operator(s).context("{s} is not in valid NameOp form")?;
    let op = operator.parse().context("parsing {s} as NameOp")?;
    Ok((name, op))
}

pub fn get_name_operator(s: &str) -> Option<(&str, &str, &str)> {
    regex_captures!(r"(\S+)(\^[-+\d]+)", s)
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PeeringSet {
    pub body: String,
    pub peerings: Vec<Peering>,
}
