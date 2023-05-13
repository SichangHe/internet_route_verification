use std::collections::BTreeMap;

use crate::lex::{dump::Dump, rpsl_object};

use super::{
    aut_num::AutNum,
    set::{AsSet, RouteSet},
};

pub fn parse_lexed(
    lexed: Dump,
) -> (
    BTreeMap<usize, AutNum>,
    BTreeMap<String, AsSet>,
    BTreeMap<String, RouteSet>,
) {
    let Dump {
        aut_nums,
        as_sets,
        route_sets,
    } = lexed;
    (
        parse_lexed_aut_nums(aut_nums),
        parse_lexed_as_sets(as_sets),
        parse_lexed_route_sets(route_sets),
    )
}

pub fn parse_lexed_aut_nums(lexed: Vec<rpsl_object::AutNum>) -> BTreeMap<usize, AutNum> {
    todo!("{lexed:?}")
}

pub fn parse_lexed_as_sets(lexed: Vec<rpsl_object::AsOrRouteSet>) -> BTreeMap<String, AsSet> {
    todo!("{lexed:?}")
}

pub fn parse_lexed_route_sets(lexed: Vec<rpsl_object::AsOrRouteSet>) -> BTreeMap<String, RouteSet> {
    todo!("{lexed:?}")
}
