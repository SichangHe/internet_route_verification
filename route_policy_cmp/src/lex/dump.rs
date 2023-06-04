use std::io::Read;

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
    pub fn from_reader(reader: impl Read) -> Result<Dump, serde_json::Error> {
        let mut deserializer = serde_json::Deserializer::from_reader(reader);
        deserializer.disable_recursion_limit();
        Dump::deserialize(&mut deserializer)
    }

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
