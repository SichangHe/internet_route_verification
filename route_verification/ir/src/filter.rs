use lazy_regex::regex_is_match;
use log::warn;

use super::*;
use RangeOperator::NoOp;

/// Parse a basic <filter> component.
pub fn parse_path_attribute(attr: String, counts: &mut Counts) -> Filter {
    if is_any(&attr) {
        Filter::Any
    } else if is_peer_as(&attr) {
        Filter::PeerAS
    } else if is_filter_set(&attr) {
        Filter::FilterSet(attr)
    } else if let Some(filter) = try_parse_route_set(&attr) {
        filter
    } else if let Some(filter) = try_parse_as_set(&attr) {
        filter
    } else if let Some(filter) = try_parse_as_num(&attr) {
        filter
    } else {
        counts.parse_path_attr += 1;
        warn!("parse_path_attribute: Unknown filter: {attr}.");
        Filter::Unknown(attr)
    }
}

pub fn is_any(attr: &str) -> bool {
    regex_is_match!(r"^(as-)?any$"i, attr)
}

pub fn is_peer_as(attr: &str) -> bool {
    regex_is_match!(r"^peeras$"i, &attr)
}

pub fn is_filter_set(attr: &str) -> bool {
    regex!(formatcp!("^{}$", FILTER_SET)).is_match(attr)
}

pub fn try_parse_route_set(attr: &str) -> Option<Filter> {
    regex!(formatcp!(r"^({})({})?$", ROUTE_SET, RANGE_OPERATOR))
        .captures(attr)
        .and_then(|caps| {
            let name = &caps[1];
            caps.get(2)
                .map_or(Some(NoOp), |operator| operator.as_str().parse().ok())
                .map(|op| Filter::RouteSet(name.into(), op))
        })
}

pub fn try_parse_as_set(attr: &str) -> Option<Filter> {
    regex!(formatcp!(r"^({})({})?$", AS_SET, RANGE_OPERATOR))
        .captures(attr)
        .and_then(|caps| {
            let name = &caps[1];
            caps.get(2)
                .map_or(Some(NoOp), |operator| operator.as_str().parse().ok())
                .map(|op| Filter::AsSet(name.into(), op))
        })
}

pub fn try_parse_as_num(attr: &str) -> Option<Filter> {
    let caps = regex!(formatcp!(r"^as([0-9]+)({})?$", RANGE_OPERATOR)).captures(attr)?;
    let num = caps.get(1)?.as_str().parse().ok()?;
    let op = match caps.get(2) {
        Some(operator) => operator.as_str().parse().ok()?,
        None => NoOp,
    };
    Some(Filter::AsNum(num, op))
}

/// > The filter attribute defines the set's policy filter.  A policy
/// > filter is a logical expression which when applied to a set of routes
/// > returns a subset of these routes.  We say that the policy filter
/// > matches the subset returned.  The policy filter can match routes
/// > using any BGP path attribute, such as the destination address prefix
/// > (or NLRI), AS-path, or community attributes.
///
/// <https://www.rfc-editor.org/rfc/rfc2622#section-5.4>
///
/// > Range operators can also be applied to address prefix sets.  In this
/// > case, they distribute over the members of the set.  For example, for
/// > a route-set (defined later) rs-foo, rs-foo^+ contains all the
/// > inclusive more specifics of all the prefixes in rs-foo.
///
/// <https://www.rfc-editor.org/rfc/rfc2622#page-5>
///
/// Note: although `RouteSet`, `AsNum`, and `AsSet` here use `RangeOperator`,
/// the RFC only allows `^-` and `^+`.
#[derive(Clone, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Filter {
    /// `<filter-set-name>`: An RPSL name that starts with `fltr-`.
    FilterSet(String),
    Any,
    /// > Address-Prefix Set This is an explicit list of address prefixes
    /// >    enclosed in braces '{' and '}'.  The policy filter matches the set
    /// >    of routes whose destination address-prefix is in the set.
    ///
    /// > An address prefix can be optionally followed by a range operator
    AddrPrefixSet(Vec<AddrPfxRange>),
    /// > Route Set Name  A route set name matches the set of routes that are
    /// > members of the set.  A route set name may be a name of a route-set
    /// > object, an AS number, or a name of an as-set object (AS numbers and
    /// > as-set names implicitly define route sets; please see Section 5.3).
    ///
    /// > A route set name can also be followed by one of the operators '^-',
    /// > '^+'…
    RouteSet(String, RangeOperator),
    /// An AS number.
    AsNum(u32, RangeOperator),
    /// A name of an as-set object.
    AsSet(String, RangeOperator),
    /// > AS Path Regular Expressions
    /// >    An AS-path regular expression can be used as a policy filter by
    /// >    enclosing the expression in '<' and '>'.  An AS-path policy filter
    /// >    matches the set of routes which traverses a sequence of ASes
    /// >    matched by the AS-path regular expression.
    AsPathRE(String),
    /// > PeerAS can be used instead of the AS number of the peer AS.
    PeerAS,
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
    Unknown(String),
}

impl std::fmt::Debug for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Filter::*;
        match self {
            FilterSet(arg0) => f.debug_tuple("FilterSet").field(arg0).finish(),
            Any => write!(f, "Any"),
            AddrPrefixSet(arg0) => f.debug_tuple("AddrPrefixSet").field(arg0).finish(),
            RouteSet(arg0, arg1) => {
                let mut r = f.debug_tuple("RouteSet");
                r.field(arg0);
                if *arg1 != RangeOperator::NoOp {
                    r.field(arg1);
                }
                r.finish()
            }
            AsNum(arg0, arg1) => {
                let mut r = f.debug_tuple("AsNum");
                r.field(arg0);
                if *arg1 != RangeOperator::NoOp {
                    r.field(arg1);
                }
                r.finish()
            }
            AsSet(arg0, arg1) => {
                let mut r = f.debug_tuple("AsSet");
                r.field(arg0);
                if *arg1 != RangeOperator::NoOp {
                    r.field(arg1);
                }
                r.finish()
            }
            AsPathRE(arg0) => f.debug_tuple("AsPathRE").field(arg0).finish(),
            PeerAS => write!(f, "PeerAS"),
            And { left, right } => f
                .debug_struct("And")
                .field("left", left)
                .field("right", right)
                .finish(),
            Or { left, right } => f
                .debug_struct("Or")
                .field("left", left)
                .field("right", right)
                .finish(),
            Not(arg0) => f.debug_tuple("Not").field(arg0).finish(),
            Group(arg0) => f.debug_tuple("Group").field(arg0).finish(),
            Community(arg0) => f.debug_tuple("Community").field(arg0).finish(),
            Unknown(arg0) => f.debug_tuple("Unknown").field(arg0).finish(),
        }
    }
}
