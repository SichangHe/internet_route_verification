use super::*;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Dump {
    pub aut_nums: Vec<AutNum>,
    pub as_sets: Vec<AsOrRouteSet>,
    pub route_sets: Vec<AsOrRouteSet>,
    pub peering_sets: Vec<PeeringSet>,
    pub filter_sets: Vec<FilterSet>,
    /// The AS in uppercase with Vec of their routes.
    pub as_routes: BTreeMap<String, Vec<String>>,
}

impl Dump {
    pub fn log_count(&self) {
        let Self {
            aut_nums,
            as_sets,
            route_sets,
            peering_sets,
            filter_sets,
            as_routes,
        } = self;
        debug!(
            "Parsed {} aut_nums, {} as_sets, {} route_sets, {} peering_sets, {} filter_sets, {} as_routes.",
            aut_nums.len(),
            as_sets.len(),
            route_sets.len(),
            peering_sets.len(),
            filter_sets.len(),
            as_routes.len(),
        )
    }
}
