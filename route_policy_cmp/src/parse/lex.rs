use std::collections::BTreeMap;

use anyhow::{bail, Context, Error, Result};
use ipnet::IpNet;
use lazy_regex::regex_captures;
use log::{debug, error};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use super::{
    aut_num::AutNum,
    aut_sys::{is_as_set, parse_as_name},
    dump::Dump,
    mp_import::parse_imports,
    peering::{is_peering_set, parse_mp_peering},
    set::{is_route_set_name, AsSet, FilterSet, PeeringSet, RouteSet},
};
use crate::{
    lex::{dump, rpsl_object},
    parse::filter::{is_filter_set, parse_filter},
};

pub fn parse_lexed(lexed: dump::Dump) -> Dump {
    debug!("Start to parse lexed dump.");
    let dump::Dump {
        aut_nums,
        as_sets,
        route_sets,
        peering_sets,
        filter_sets,
        as_routes,
    } = lexed;
    let dump = Dump {
        aut_nums: parse_lexed_aut_nums(aut_nums),
        as_sets: parse_lexed_as_sets(as_sets),
        route_sets: parse_lexed_route_sets(route_sets),
        peering_sets: parse_lexed_peering_sets(peering_sets),
        filter_sets: parse_lexed_filter_sets(filter_sets),
        as_routes: parse_lexed_as_routes(as_routes),
    };
    dump.log_count();
    dump
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
        None => bail!("AS number name `{name}` does not match pattern"),
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
        bail!("invalid AS Set name in {lexed:?}");
    }
    let members = match lexed
        .members
        .into_iter()
        .map(parse_as_name)
        .collect::<Result<Vec<_>>>()
    {
        Ok(m) => m,
        Err(err) => {
            return Err(err.context(format!("parsing AS Set {}\n{}", lexed.name, lexed.body)))
        }
    };
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
            "{} is an invalid route set name—parsing {lexed:?}",
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
            "{} is an invalid peering set name—parsing {lexed:?}",
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

pub fn parse_lexed_filter_sets(lexed: Vec<rpsl_object::FilterSet>) -> BTreeMap<String, FilterSet> {
    lexed
        .into_par_iter()
        .filter_map(|l| parse_lexed_filter_set(l).map_err(|e| error!("{e:#}")).ok())
        .collect()
}

pub fn parse_lexed_filter_set(lexed: rpsl_object::FilterSet) -> Result<(String, FilterSet)> {
    if !is_filter_set(&lexed.name) {
        bail!(
            "{} is an invalid filter set name—parsing {lexed:?}",
            lexed.name
        );
    }
    let filter_set = FilterSet {
        body: lexed.body,
        filters: lexed
            .filters
            .into_iter()
            .map(|f| parse_filter(f, &[]))
            .collect(),
    };
    Ok((lexed.name, filter_set))
}

pub fn parse_lexed_as_routes(
    as_routes: BTreeMap<String, Vec<String>>,
) -> BTreeMap<usize, Vec<IpNet>> {
    as_routes
        .into_iter()
        .filter_map(|as_route| {
            parse_lexed_as_route(&as_route)
                .map_err(|e| error!("Parsing routes for {as_route:?}: {e}."))
                .ok()
        })
        .collect()
}

pub fn parse_lexed_as_route((name, routes): &(String, Vec<String>)) -> Result<(usize, Vec<IpNet>)> {
    let num = parse_aut_num_name(name)?;
    let routes: Result<_> = routes.iter().map(|r| Ok(r.parse()?)).collect();
    Ok((num, routes?))
}
