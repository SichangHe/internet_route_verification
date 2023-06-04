use std::collections::BTreeMap;

use anyhow::{bail, Context, Error, Result};
use lazy_regex::regex_captures;
use log::{error, info};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use super::{
    aut_num::AutNum,
    aut_sys::{is_as_set, parse_as_name},
    mp_import::parse_imports,
    peering::{is_peering_set, parse_mp_peering},
    set::{is_route_set_name, AsSet, PeeringSet, RouteSet},
};
use crate::lex::{dump, rpsl_object};

pub fn parse_lexed(lexed: dump::Dump) -> Dump {
    info!("Start to parse lexed dump.");
    let dump::Dump {
        aut_nums,
        as_sets,
        route_sets,
        peering_sets,
    } = lexed;
    let aut_nums = parse_lexed_aut_nums(aut_nums);
    info!("Parsed {} Aut Nums.", aut_nums.len());
    let as_sets = parse_lexed_as_sets(as_sets);
    info!("Parsed {} As Sets.", as_sets.len());
    let route_sets = parse_lexed_route_sets(route_sets);
    info!("Parsed {} Route Sets.", route_sets.len());
    let peering_sets = parse_lexed_peering_sets(peering_sets);
    info!("Parsed {} Peering Sets.", peering_sets.len());
    Dump {
        aut_nums,
        as_sets,
        route_sets,
        peering_sets,
    }
}

pub fn parse_lexed_aut_nums(lexed: Vec<rpsl_object::AutNum>) -> BTreeMap<usize, AutNum> {
    lexed
        .into_par_iter()
        .filter_map(|l| parse_lexed_aut_num(l).map_err(|e| error!("{e:#}")).ok())
        .collect()
}

pub fn parse_lexed_aut_num(aut_num: rpsl_object::AutNum) -> Result<(usize, AutNum)> {
    let num = parse_aut_num_name(&aut_num.name).context(format!("parsing {aut_num:?}"))?;
    let rpsl_object::AutNum {
        name: _,
        body,
        imports,
        exports,
    } = aut_num;
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
            .map_err(|err| Error::new(err).context(format!("parsing {name}"))),
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
    let members = lexed
        .members
        .into_iter()
        .map(|member| member.into())
        .collect();

    Ok((
        lexed.name,
        RouteSet {
            body: lexed.body,
            members,
        },
    ))
}

pub fn parse_lexed_peering_sets(
    lexed: Vec<rpsl_object::PeeringSet>,
) -> BTreeMap<String, PeeringSet> {
    lexed
        .into_par_iter()
        .filter_map(|l| parse_lexed_peering_set(l).map_err(|e| error!("{e:#}")).ok())
        .collect()
}

pub fn parse_lexed_peering_set(lexed: rpsl_object::PeeringSet) -> Result<(String, PeeringSet)> {
    if !is_peering_set(&lexed.name) {
        bail!(
            "{} is an illegal ppeering set name—parsing {lexed:?}",
            lexed.name
        );
    }
    Ok((
        lexed.name,
        PeeringSet {
            body: lexed.body,
            peerings: lexed.peerings.into_iter().map(parse_mp_peering).collect(),
        },
    ))
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Dump {
    pub aut_nums: BTreeMap<usize, AutNum>,
    pub as_sets: BTreeMap<String, AsSet>,
    pub route_sets: BTreeMap<String, RouteSet>,
    pub peering_sets: BTreeMap<String, PeeringSet>,
}
