use std::collections::BTreeMap;

use ipnet::IpNet;
use rayon::prelude::*;

use crate::parse::{aut_num::AutNum, dump::Dump, set::*};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AsSetRoute {
    /// This field should always be sorted.
    pub routes: Vec<IpNet>,
    pub unrecorded_nums: Vec<usize>,
    pub set_members: Vec<String>,
}

impl AsSetRoute {
    pub fn from_as_set(as_set: &AsSet, as_routes: &BTreeMap<usize, Vec<IpNet>>) -> Self {
        let mut routes = Vec::with_capacity(as_set.members.len() << 2);
        let mut unrecorded_nums = Vec::new();
        for member in &as_set.members {
            match as_routes.get(member) {
                Some(as_route) => routes.extend(as_route),
                None => unrecorded_nums.push(*member),
            }
        }
        routes.shrink_to_fit();
        routes.sort_unstable();
        unrecorded_nums.shrink_to_fit();
        unrecorded_nums.sort_unstable();
        Self {
            routes,
            unrecorded_nums,
            set_members: as_set.set_members.clone(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct QueryDump {
    pub aut_nums: BTreeMap<usize, AutNum>,
    pub as_sets: BTreeMap<String, AsSet>,
    pub route_sets: BTreeMap<String, RouteSet>,
    pub peering_sets: BTreeMap<String, PeeringSet>,
    pub filter_sets: BTreeMap<String, FilterSet>,
    /// Each value should always be sorted.
    pub as_routes: BTreeMap<usize, Vec<IpNet>>,
    /// Each value should always be sorted.
    pub as_set_routes: BTreeMap<String, AsSetRoute>,
}

impl QueryDump {
    pub fn from_dump(dump: Dump) -> Self {
        let Dump {
            aut_nums,
            as_sets,
            route_sets,
            peering_sets,
            filter_sets,
            as_routes,
        } = dump;
        let as_set_routes = as_sets
            .par_iter()
            .map(|(name, set)| (name.clone(), AsSetRoute::from_as_set(set, &as_routes)))
            .collect();
        Self {
            aut_nums,
            as_sets,
            route_sets,
            peering_sets,
            filter_sets,
            as_routes,
            as_set_routes,
        }
    }
}
