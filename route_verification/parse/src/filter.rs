use ::lex;
use ir::filter::parse_path_attribute;

use super::*;

pub fn parse_filter(
    mp_filter: lex::Filter,
    mp_peerings: &[PeeringAction],
    counts: &mut Counts,
) -> Filter {
    use lex::Filter::*;
    match mp_filter {
        Any => Filter::Any,
        And { left, right } => Filter::And {
            left: Box::new(parse_filter(*left, mp_peerings, counts)),
            right: Box::new(parse_filter(*right, mp_peerings, counts)),
        },
        Or { left, right } => Filter::Or {
            left: Box::new(parse_filter(*left, mp_peerings, counts)),
            right: Box::new(parse_filter(*right, mp_peerings, counts)),
        },
        Not(filter) => Filter::Not(Box::new(parse_filter(*filter, mp_peerings, counts))),
        Group(group) => Filter::Group(Box::new(parse_filter(*group, mp_peerings, counts))),
        Community(call) => Filter::Community(call),
        PathAttr(attr) => parse_path_attribute(attr, mp_peerings, counts),
        AddrPrefixSet(set) => Filter::AddrPrefixSet(
            set.into_iter()
                .filter_map(|s| {
                    s.parse() // We expect low a chance of error here.
                        .map_err(|e| error!("parsing {s} as address prefix: {e:?}"))
                        .ok()
                })
                .collect(),
        ),
        Regex(expr) => Filter::AsPathRE(expr),
    }
}
