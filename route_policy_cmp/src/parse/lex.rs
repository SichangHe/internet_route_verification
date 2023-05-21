use std::{collections::BTreeMap, io::Read};

use anyhow::{bail, Context, Error, Result};
use lazy_regex::regex_captures;
use log::{error, info};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use super::{
    aut_num::AutNum,
    aut_sys::{is_as_set, parse_as_name},
    mp_import::parse_imports,
    set::{is_route_set_name, AsSet, RouteSet},
};
use crate::lex::{dump, rpsl_object};

pub fn parse_lexed(lexed: dump::Dump) -> Dump {
    info!("Start to parse lexed dump.");
    let dump::Dump {
        aut_nums,
        as_sets,
        route_sets,
    } = lexed;
    let aut_nums = parse_lexed_aut_nums(aut_nums);
    info!("Parsed {} Aut Nums.", aut_nums.len());
    let as_sets = parse_lexed_as_sets(as_sets);
    info!("Parsed {} As Sets.", aut_nums.len());
    let route_sets = parse_lexed_route_sets(route_sets);
    info!("Parsed {} Route Sets.", aut_nums.len());
    Dump {
        aut_nums,
        as_sets,
        route_sets,
    }
}

pub fn parse_lexed_aut_nums(lexed: Vec<rpsl_object::AutNum>) -> BTreeMap<usize, AutNum> {
    lexed
        .into_par_iter()
        .map(parse_lexed_aut_num)
        .fold(BTreeMap::new, |mut parsed, result| {
            match result {
                Ok((num, aut_num)) => {
                    parsed.insert(num, aut_num);
                }
                Err(err) => error!("{err:#}"),
            }
            parsed
        })
        .reduce(BTreeMap::new, |mut they, we| {
            they.extend(we);
            they
        })
}

pub fn parse_lexed_aut_num(aut_num: rpsl_object::AutNum) -> Result<(usize, AutNum)> {
    let rpsl_object::AutNum {
        name,
        body,
        imports,
        exports,
    } = aut_num;
    let num = parse_aut_num_name(&name).context("parsing {aut_num:?}")?;
    let imports = parse_imports(imports);
    let exports = parse_imports(exports);
    Ok((
        num,
        AutNum {
            body,
            imports,
            exports,
        },
    ))
}

pub fn parse_aut_num_name(name: &str) -> Result<usize> {
    match regex_captures!(r"^AS(\d+)$"i, name) {
        Some((_, num)) => num
            .parse()
            .map_err(|err| Error::new(err).context("parsing {name}")),
        None => bail!("AS number name {name} does not match pattern"),
    }
}

pub fn parse_lexed_as_sets(lexed: Vec<rpsl_object::AsOrRouteSet>) -> BTreeMap<String, AsSet> {
    lexed
        .into_par_iter()
        .filter_map(|l| parse_lexed_as_set(l).map_err(|e| error!("{e:#}")).ok())
        .collect()
}

pub fn parse_lexed_as_set(lexed: rpsl_object::AsOrRouteSet) -> Result<(String, AsSet)> {
    if !is_as_set(&lexed.name) {
        bail!("illegal AS set name in {lexed:?}");
    }
    let members = lexed.members.into_iter().map(parse_as_name).collect();
    let as_set = AsSet {
        body: lexed.body,
        members,
    };
    Ok((lexed.name, as_set))
}

pub fn parse_lexed_route_sets(lexed: Vec<rpsl_object::AsOrRouteSet>) -> BTreeMap<String, RouteSet> {
    lexed
        .into_par_iter()
        .filter_map(|l| parse_lexed_route_set(l).map_err(|e| error!("{e:#}")).ok())
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

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Dump {
    pub aut_nums: BTreeMap<usize, AutNum>,
    pub as_sets: BTreeMap<String, AsSet>,
    pub route_sets: BTreeMap<String, RouteSet>,
}

impl Dump {
    pub fn from_reader(reader: impl Read) -> Result<Dump, serde_json::Error> {
        let mut deserializer = serde_json::Deserializer::from_reader(reader);
        deserializer.disable_recursion_limit();
        Dump::deserialize(&mut deserializer)
    }
}
