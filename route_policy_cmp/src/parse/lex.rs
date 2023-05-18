use std::collections::BTreeMap;

use anyhow::{bail, Error, Result};
use lazy_regex::regex_captures;
use log::error;

use super::{
    aut_num::AutNum,
    mp_import::parse_imports,
    set::{AsSet, RouteSet},
};
use crate::lex::{dump::Dump, rpsl_object};

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
    let mut parsed = BTreeMap::new();
    for aut_num in lexed {
        let rpsl_object::AutNum {
            name,
            body,
            imports,
            exports,
        } = aut_num;
        let mut errors = Vec::new();
        let num = match parse_aut_num_name(&name) {
            Ok(num) => num,
            Err(err) => {
                error!("{err}");
                errors.push(err.to_string());
                continue;
            }
        };
        let imports = parse_imports(imports);
        let exports = parse_imports(exports);
        parsed.insert(
            num,
            AutNum {
                body,
                errors,
                imports,
                exports,
            },
        );
    }
    parsed
}

pub fn parse_aut_num_name(name: &str) -> Result<usize> {
    match regex_captures!(r"^AS(\d+)$"i, name) {
        Some((_, num)) => num
            .parse()
            .map_err(|err| Error::new(err).context("parsing {name}")),
        None => bail!("AS name does not match pattern {name}"),
    }
}

pub fn parse_lexed_as_sets(lexed: Vec<rpsl_object::AsOrRouteSet>) -> BTreeMap<String, AsSet> {
    let parsed = BTreeMap::new();
    // TODO: Implement.
    println!("{lexed:?}");
    parsed
}

pub fn parse_lexed_route_sets(lexed: Vec<rpsl_object::AsOrRouteSet>) -> BTreeMap<String, RouteSet> {
    let parsed = BTreeMap::new();
    // TODO: Implement.
    println!("{lexed:?}");
    parsed
}
