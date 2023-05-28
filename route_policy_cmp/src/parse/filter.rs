use lazy_regex::{regex_captures, regex_is_match};
use log::error;
use serde::{Deserialize, Serialize};

use crate::lex::{community::Call, filter};

use super::{address_prefix::AddrPfxRange, set::is_route_set_name};

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
        AddrPrefixSet(set) => Filter::AddrPrefixSet(
            set.into_iter()
                .filter_map(|s| {
                    s.parse() // We expect low a chance of error here.
                        .map_err(|e| error!("parsing {s} as address prefix: {e:#}"))
                        .ok()
                })
                .collect(),
        ),
        Regex(expr) => Filter::AsPathRE(expr),
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
    } else if is_route_set_name(&attr) {
        Filter::RouteSetName(attr)
    } else if let Some(filter) = try_parse_as_set(&attr) {
        filter
    } else if let Some(filter) = try_parse_as_num(&attr) {
        filter
    } else {
        Filter::AsPathRE(attr)
    }
}

pub fn try_parse_as_set(attr: &str) -> Option<Filter> {
    regex_captures!(r"^((?:AS\d+:)?AS-[^\s\^]+)(\^[+-])?$"i, attr).and_then(
        |(_, name, operator)| {
            RegexOperator::parse_str(operator).map(|op| Filter::AsSet(name.into(), op))
        },
    )
}

pub fn try_parse_as_num(attr: &str) -> Option<Filter> {
    regex_captures!(r"^AS(\d+)(\^[+-])?$"i, attr).and_then(|(_, number, operator)| {
        RegexOperator::parse_str(operator)
            .and_then(|op| number.parse().ok().map(|num| Filter::AsNum(num, op)))
    })
}

/// <https://www.rfc-editor.org/rfc/rfc2622#section-5.4>
/// <https://www.rfc-editor.org/rfc/rfc2622#page-18>
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Filter {
    /// `<filter-set-name>`: An RPSL name that starts with `fltr-`.
    FilterSetName(String),
    Any,
    /// An explicit list of address prefixes enclosed in braces '{' and '}'.  The policy filter matches the set of routes whose destination address-prefix is in the set.
    /// An address prefix can be optionally followed by a range operator.
    AddrPrefixSet(Vec<AddrPfxRange>),
    /// `<route-set-name>`: <https://www.rfc-editor.org/rfc/rfc2622#section-5.2>.
    /// A route set name matches the set of routes that are members of the set.
    /// May also be implicitly defined route sets
    /// <https://www.rfc-editor.org/rfc/rfc2622#section-5.3>.
    RouteSetName(String),
    /// An AS number.
    AsNum(usize, RegexOperator),
    /// A name of an as-set object.
    AsSet(String, RegexOperator),
    /// An AS-path regular expression can be used as a policy filter by enclosing the expression in `<' and `>'.
    /// Basically, we do not deal with this at present.
    /// We also throw unrecognized filters under this.
    /// <https://www.rfc-editor.org/rfc/rfc2622#page-19>.
    AsPathRE(String),
    /// Can be used instead of the AS number of the peer AS.
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

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RegexOperator {
    NoOp,
    /// `^+`
    Plus,
    /// `^-`
    Minus,
}

impl RegexOperator {
    pub fn parse_str(s: &str) -> Option<Self> {
        match s {
            "" => Some(Self::NoOp),
            "^+" => Some(Self::Plus),
            "^-" => Some(Self::Minus),
            _ => None,
        }
    }
}
