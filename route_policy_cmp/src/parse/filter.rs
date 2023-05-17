use serde::{Deserialize, Serialize};

use crate::lex::{community::Call, filter};

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
    match attr.to_lowercase().as_str() {
        "any" => Filter::Any,
        "peeras" => Filter::PeerAs,
        _ => todo!(),
    }
}

/// <https://www.rfc-editor.org/rfc/rfc2622#section-5.4>
/// <https://www.rfc-editor.org/rfc/rfc2622#page-18>
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Filter {
    /// An RPSL name that starts with `fltr-`.
    // TODO: Recognize <filter-set>s.
    FilterSet(String),
    Any,
    // TODO: Parse address prefixes.
    AddrPrefixSet(Vec<String>),
    /// May also be implicitly define route sets
    /// <https://www.rfc-editor.org/rfc/rfc2622#section-5.3>.
    RouteSet(String),
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
