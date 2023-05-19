use std::collections::BTreeMap;

use anyhow::{anyhow, bail, Error, Result};
use lazy_regex::regex_captures;
use log::error;

use super::{
    aut_num::AutNum,
    aut_sys::{parse_as_name, try_parse_as_set},
    mp_import::parse_imports,
    set::{is_route_set_name, AsSet, RouteSet},
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
                let err = format!("{err:?}");
                error!("{err}");
                errors.push(err);
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
    lexed
        .into_iter()
        .filter_map(|l| parse_lexed_as_set(l).map_err(|e| error!("{e:?}")).ok())
        .collect()
}

pub fn parse_lexed_as_set(lexed: rpsl_object::AsOrRouteSet) -> Result<(String, AsSet)> {
    let name = try_parse_as_set(&lexed.name)
        .ok_or_else(|| anyhow!("{} is an illegal AS set name—parsing {lexed:?}", lexed.name))?;
    let members = lexed
        .members
        .into_iter()
        .map(|f| parse_as_name(&f))
        .collect();
    let as_set = AsSet {
        body: lexed.body,
        members,
    };
    Ok((name.into(), as_set))
}

pub fn parse_lexed_route_sets(lexed: Vec<rpsl_object::AsOrRouteSet>) -> BTreeMap<String, RouteSet> {
    lexed
        .into_iter()
        .filter_map(|l| parse_lexed_route_set(l).map_err(|e| error!("{e:?}")).ok())
        .collect()
}

pub fn parse_lexed_route_set(lexed: rpsl_object::AsOrRouteSet) -> Result<(String, RouteSet)> {
    if !is_route_set_name(&lexed.name) {
        bail!(
            "{} is an illegal route set name—parsing {lexed:?}",
            lexed.name
        );
    }
    Ok((
        lexed.name,
        RouteSet {
            body: lexed.body,
            members: lexed.members,
        },
    ))
}
