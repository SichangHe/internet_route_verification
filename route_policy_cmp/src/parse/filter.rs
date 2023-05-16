use serde::{Deserialize, Serialize};

use crate::lex::{
    community::Call,
    filter::{self, Base},
};

pub fn parse_filter(mp_filter: filter::Filter) -> Filter {
    match mp_filter {
        filter::Filter::Mixed(base) => parse_filter_base(base),
        filter::Filter::Policies(policies) => parse_filter_policies(policies),
    }
}

pub fn parse_filter_base(base: filter::Base) -> Filter {
    use Filter::*;
    match base {
        Base::And { left, right } => And {
            left: Box::new(parse_filter(*left)),
            right: Box::new(parse_filter(*right)),
        },
        Base::Or { left, right } => Or {
            left: Box::new(parse_filter(*left)),
            right: Box::new(parse_filter(*right)),
        },
        Base::Not(filter) => Not(Box::new(parse_filter(*filter))),
        Base::Group(group) => Group(Box::new(parse_filter(*group))),
        Base::Community(call) => Community(call),
    }
}

pub fn parse_filter_policies(policies: Vec<filter::Policy>) -> Filter {
    todo!()
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
