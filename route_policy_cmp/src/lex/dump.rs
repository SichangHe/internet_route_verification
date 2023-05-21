use std::io::Read;

use serde::{Deserialize, Serialize};

use super::rpsl_object::{AsOrRouteSet, AutNum};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Dump {
    pub aut_nums: Vec<AutNum>,
    pub as_sets: Vec<AsOrRouteSet>,
    pub route_sets: Vec<AsOrRouteSet>,
}

impl Dump {
    pub fn from_reader(reader: impl Read) -> Result<Dump, serde_json::Error> {
        let mut deserializer = serde_json::Deserializer::from_reader(reader);
        deserializer.disable_recursion_limit();
        Dump::deserialize(&mut deserializer)
    }
}
