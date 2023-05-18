use lazy_regex::regex_is_match;
use serde::{Deserialize, Serialize};

use crate::lex::{community::Call, filter};

use super::{
    lex::parse_aut_num_name,
    peering::{try_parse_as_set, AsExpr},
};

pub fn parse_filter(mp_filter: filter::Filter) -> Filter {
    use filter::Filter::*;
    match mp_filter {
        And { left, right } => Filter::And {
            left: Box::new(parse_filter(*left)),
            right: Box::new(parse_filter(*right)),
        },
        Or { left, right } => Filter::Or {
            left: Box::new(parse_filter(*left)),
            right: Box::new(parse_filter(*right)),
        },
        Not(filter) => Filter::Not(Box::new(parse_filter(*filter))),
        Group(group) => Filter::Group(Box::new(parse_filter(*group))),
        Community(call) => Filter::Community(call),
        PathAttr(attr) => parse_path_attribute(attr),
        AddrPrefixSet(set) => Filter::AddrPrefixSet(set),
    }
}

pub fn parse_path_attribute(attr: String) -> Filter {
    if regex_is_match!(r"^any$"i, &attr) {
        Filter::Any
    } else if regex_is_match!(r"^peeras$"i, &attr) {
        Filter::PeerAs
    } else if attr.ends_with("^-") || attr.ends_with("^+") {
        Filter::AsPathRE(attr)
    } else if regex_is_match!(r"^(AS\d+:)?fltr-\S+$"i, &attr) {
        Filter::FilterSetName(attr)
    } else if regex_is_match!(r"^(AS\d+:)?rs-\S+$"i, &attr) {
        Filter::RouteSetName(attr)
    } else if let Some(name) = try_parse_as_set(&attr) {
        Filter::AsSet(name.into())
    } else if let Ok(num) = parse_aut_num_name(&attr) {
        Filter::AsNum(num)
    } else {
        Filter::AsPathRE(attr)
    }
}

/// <https://www.rfc-editor.org/rfc/rfc2622#section-5.4>
/// <https://www.rfc-editor.org/rfc/rfc2622#page-18>
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Filter {
    /// `<filter-set-name>`: An RPSL name that starts with `fltr-`.
    FilterSetName(String),
    Any,
    // TODO: Parse address prefixes.
    AddrPrefixSet(Vec<String>),
    /// `<route-set-name>`: <https://www.rfc-editor.org/rfc/rfc2622#section-5.2>.
    /// May also be implicitly define route sets
    /// <https://www.rfc-editor.org/rfc/rfc2622#section-5.3>.
    RouteSetName(String),
    AsNum(usize),
    AsSet(String),
    /// <https://www.rfc-editor.org/rfc/rfc2622#page-19>.
    AsPathRE(String),
    PeerAs,
    And {
        left: Box<Filter>,
        right: Box<Filter>,
    },
    Or {
        left: Box<Filter>,
        right: Box<Filter>,
    },
    Not(Box<Filter>),
    Group(Box<Filter>),
    Community(Call),
}
