use lazy_regex::{regex_captures, regex_is_match};
use log::error;
use serde::{Deserialize, Serialize};

use crate::lex::{community::Call, filter};

use super::{
    address_prefix::{
        AddrPfxRange,
        RangeOperator::{self, NoOp},
    },
    aut_sys::AsName,
    peering::{AsExpr, Peering, PeeringAction},
};

pub fn parse_filter(mp_filter: filter::Filter, mp_peerings: &[PeeringAction]) -> Filter {
    use filter::Filter::*;
    match mp_filter {
        And { left, right } => Filter::And {
            left: Box::new(parse_filter(*left, mp_peerings)),
            right: Box::new(parse_filter(*right, mp_peerings)),
        },
        Or { left, right } => Filter::Or {
            left: Box::new(parse_filter(*left, mp_peerings)),
            right: Box::new(parse_filter(*right, mp_peerings)),
        },
        Not(filter) => Filter::Not(Box::new(parse_filter(*filter, mp_peerings))),
        Group(group) => Filter::Group(Box::new(parse_filter(*group, mp_peerings))),
        Community(call) => Filter::Community(call),
        PathAttr(attr) => parse_path_attribute(attr, mp_peerings),
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

pub fn parse_path_attribute(attr: String, mp_peerings: &[PeeringAction]) -> Filter {
    if regex_is_match!(r"^any$"i, &attr) {
        Filter::Any
    } else if regex_is_match!(r"^peeras$"i, &attr) {
        peer_as_filter(mp_peerings)
    } else if attr.ends_with("^-") || attr.ends_with("^+") {
        Filter::AsPathRE(attr)
    } else if is_filter_set(&attr) {
        Filter::FilterSet(attr)
    } else if let Some(filter) = try_parse_route_set(&attr) {
        filter
    } else if let Some(filter) = try_parse_as_set(&attr) {
        filter
    } else if let Some(filter) = try_parse_as_num(&attr) {
        filter
    } else {
        Filter::AsPathRE(attr)
    }
}

pub fn is_filter_set(attr: &str) -> bool {
    regex_is_match!(r"^(AS\d+:)?fltr-\S+$"i, attr)
}

/// PeerAS can be used instead of the AS number of the peer AS.
/// <https://www.rfc-editor.org/rfc/rfc2622#page-19>.
pub fn peer_as_filter(mp_peerings: &[PeeringAction]) -> Filter {
    match (mp_peerings.len(), mp_peerings.first()) {
        (
            1,
            Some(PeeringAction {
                mp_peering:
                    Peering {
                        remote_as: AsExpr::Single(as_name),
                        remote_router: _,
                        local_router: _,
                    },
                actions: _,
            }),
        ) => match as_name {
            AsName::Num(num) => Filter::AsNum(*num, NoOp),
            AsName::Set(name) => Filter::AsSet(name.into(), NoOp),
            AsName::Invalid(reason) => {
                let err = format!("PeerAs point to invalid AS name: {reason}.");
                error!("{err}");
                Filter::Invalid(err)
            }
        },
        _ => Filter::Invalid(format!(
            "using PeerAs but mp-peerings {mp_peerings:?} are not a single AS expression"
        )),
    }
}

pub fn try_parse_route_set(attr: &str) -> Option<Filter> {
    regex_captures!(r"^((?:AS\d+:)?rs-[^\s\^]+)(\^[+-])?$"i, attr).and_then(
        |(_, name, operator)| {
            operator
                .parse()
                .ok()
                .map(|op| Filter::RouteSet(name.into(), op))
        },
    )
}

pub fn try_parse_as_set(attr: &str) -> Option<Filter> {
    regex_captures!(r"^((?:AS\d+:)?AS-[^\s\^]+)(\^[+-])?$"i, attr).and_then(
        |(_, name, operator)| {
            operator
                .parse()
                .ok()
                .map(|op| Filter::AsSet(name.into(), op))
        },
    )
}

pub fn try_parse_as_num(attr: &str) -> Option<Filter> {
    regex_captures!(r"^AS(\d+)(\^[+-])?$"i, attr).and_then(|(_, number, operator)| {
        operator
            .parse()
            .ok()
            .and_then(|op| number.parse().ok().map(|num| Filter::AsNum(num, op)))
    })
}

/// <https://www.rfc-editor.org/rfc/rfc2622#section-5.4>
/// <https://www.rfc-editor.org/rfc/rfc2622#page-18>
/// Note: although `RouteSet`, `AsNum`, and `AsSet` here use `RangeOperator`,
/// the RFC only allows `^-` and `^+`.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Filter {
    /// `<filter-set-name>`: An RPSL name that starts with `fltr-`.
    FilterSet(String),
    Any,
    /// An explicit list of address prefixes enclosed in braces '{' and '}'.  The policy filter matches the set of routes whose destination address-prefix is in the set.
    /// An address prefix can be optionally followed by a range operator.
    AddrPrefixSet(Vec<AddrPfxRange>),
    /// `<route-set-name>`: <https://www.rfc-editor.org/rfc/rfc2622#section-5.2>.
    /// A route set name matches the set of routes that are members of the set.
    /// May also be implicitly defined route sets
    /// <https://www.rfc-editor.org/rfc/rfc2622#section-5.3>.
    RouteSet(String, RangeOperator),
    /// An AS number.
    AsNum(usize, RangeOperator),
    /// A name of an as-set object.
    AsSet(String, RangeOperator),
    /// An AS-path regular expression can be used as a policy filter by enclosing the expression in `<' and `>'.
    /// Basically, we do not deal with this at present.
    /// We also throw unrecognized filters under this.
    /// <https://www.rfc-editor.org/rfc/rfc2622#page-19>.
    AsPathRE(String),
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
    Invalid(String),
}
