use serde::{Deserialize, Serialize};

use super::rpsl_object::{AsOrRouteSet, AutNum};

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Dump {
    pub aut_nums: Vec<AutNum>,
    pub as_sets: Vec<AsOrRouteSet>,
    pub route_sets: Vec<AsOrRouteSet>,
}
