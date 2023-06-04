use log::debug;
use serde::{Deserialize, Serialize};

use super::rpsl_object::{AsOrRouteSet, AutNum, PeeringSet};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Dump {
    pub aut_nums: Vec<AutNum>,
    pub as_sets: Vec<AsOrRouteSet>,
    pub route_sets: Vec<AsOrRouteSet>,
    pub peering_sets: Vec<PeeringSet>,
}

impl Dump {
    pub fn log_count(&self) {
        debug!(
            "Parsed {} aut_nums, {} as_sets, {} route_sets, {} peering_sets.",
            self.aut_nums.len(),
            self.as_sets.len(),
            self.route_sets.len(),
            self.peering_sets.len()
        )
    }
}
