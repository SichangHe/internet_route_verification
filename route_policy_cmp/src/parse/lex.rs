use std::{
    collections::{BTreeMap, VecDeque},
    fs::{create_dir_all, File},
    path::Path,
    thread::available_parallelism,
};

use anyhow::{bail, Context, Error, Ok, Result};
use ipnet::IpNet;
use itertools::izip;
use lazy_regex::regex_captures;
use log::{debug, error};
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use serde::{Deserialize, Serialize};

use super::{
    aut_num::AutNum,
    aut_sys::{is_as_set, parse_as_name},
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
    let aut_nums = parse_lexed_aut_nums(aut_nums);
    debug!("Parsed {} Aut Nums.", aut_nums.len());
    let as_sets = parse_lexed_as_sets(as_sets);
    debug!("Parsed {} As Sets.", as_sets.len());
    let route_sets = parse_lexed_route_sets(route_sets);
    debug!("Parsed {} Route Sets.", route_sets.len());
    let peering_sets = parse_lexed_peering_sets(peering_sets);
    debug!("Parsed {} Peering Sets.", peering_sets.len());
    let filter_sets = parse_lexed_filter_sets(filter_sets);
    debug!("Parsed {} Filter Sets.", filter_sets.len());
    let as_routes = parse_lexed_as_routes(as_routes);
    debug!("Parsed {} AS Routes.", as_routes.len());
    Dump {
        aut_nums,
        as_sets,
        route_sets,
        peering_sets,
        filter_sets,
        as_routes,
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
        bail!("invalid AS set name in {lexed:?}");
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

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Dump {
    pub aut_nums: BTreeMap<usize, AutNum>,
    pub as_sets: BTreeMap<String, AsSet>,
    pub route_sets: BTreeMap<String, RouteSet>,
    pub peering_sets: BTreeMap<String, PeeringSet>,
    pub filter_sets: BTreeMap<String, FilterSet>,
    /// The AS numbers with Vec of their routes.
    /// <https://www.rfc-editor.org/rfc/rfc2622#section-4>.
    pub as_routes: BTreeMap<usize, Vec<IpNet>>,
}

pub fn split_n_btreemap<K, V>(mut map: BTreeMap<K, V>, n: usize) -> Vec<BTreeMap<K, V>>
where
    K: std::cmp::Ord + Clone,
{
    let size_per_split = map.len() / n;
    let mut split_points = VecDeque::with_capacity(n - 1);
    for (index, (key, _)) in map.iter().enumerate() {
        if index % size_per_split == 0 {
            split_points.push_back((*key).clone());
        }
    }
    let mut splits = Vec::with_capacity(n);
    splits.push(BTreeMap::new());
    for _ in 1..n {
        let split = map.split_off(&split_points.pop_front().unwrap());
        splits.push(split);
    }
    splits[0] = map;
    splits
}

impl Dump {
    pub fn split_n(self, n: usize) -> Vec<Self> {
        let Self {
            aut_nums,
            as_sets,
            route_sets,
            peering_sets,
            filter_sets,
            as_routes,
        } = self;
        let aut_num_splits = split_n_btreemap(aut_nums, n);
        let as_set_splits = split_n_btreemap(as_sets, n);
        let route_set_splits = split_n_btreemap(route_sets, n);
        let peering_set_splits = split_n_btreemap(peering_sets, n);
        let filter_set_splits = split_n_btreemap(filter_sets, n);
        let as_route_splits = split_n_btreemap(as_routes, n);

        izip!(
            aut_num_splits,
            as_set_splits,
            route_set_splits,
            peering_set_splits,
            filter_set_splits,
            as_route_splits
        )
        .map(
            |(aut_nums, as_sets, route_sets, peering_sets, filter_sets, as_routes)| Self {
                aut_nums,
                as_sets,
                route_sets,
                peering_sets,
                filter_sets,
                as_routes,
            },
        )
        .collect()
    }

    /// Split self based on the number of CPU logic cores available × 4.
    pub fn split_n_cpus(self) -> Result<Vec<Self>> {
        let n: usize = available_parallelism()?.into();
        Ok(self.split_n(n * 4))
    }

    /// Quickly write self to `directory` in parallel.
    pub fn pal_write<P>(self, directory: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let splits = self.split_n_cpus()?;
        pal_write_dump(&splits, directory)
    }
}

pub fn pal_write_dump<P>(splits: &Vec<Dump>, directory: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let directory = directory.as_ref().to_owned();
    create_dir_all(&directory)?;
    splits
        .par_iter()
        .enumerate()
        .map(|(index, dump)| {
            let path = directory.clone().join(format!("{index}.json"));
            let file = File::create(path)?;
            serde_json::to_writer(file, dump)?;
            Ok(())
        })
        .collect::<Result<()>>()
}
