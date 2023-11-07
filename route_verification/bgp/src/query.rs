use hashbrown::HashMap;

use super::*;

mod pseudo_set;

pub use pseudo_set::*;

/// Routes for one AS set, including the unrecorded and sets.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AsSetRoute {
    /// Should always be sorted.
    pub members: Vec<u32>,
    /// Should always be sorted.
    pub routes: Vec<IpNet>,
    pub unrecorded_nums: Vec<u32>,
    pub set_members: Vec<String>,
}

impl AsSetRoute {
    /// Clean up `members`, `routes` and `unrecorded_nums` into compact ordered
    /// sets.
    pub fn clean_up(&mut self) {
        clean_vec(&mut self.members);
        clean_vec(&mut self.routes);
        clean_vec(&mut self.unrecorded_nums);
    }

    /// Fill in routes for the AS with `as_set` with routes in `as_routes`.
    /// The process is done only once, and the result [`AsSetRoute`] is cleaned.
    pub fn from_as_set(as_set: &AsSet, as_routes: &BTreeMap<u32, Vec<IpNet>>) -> Self {
        let mut routes = Vec::with_capacity(as_set.members.len() << 2);
        let mut unrecorded_nums = Vec::new();
        for member in &as_set.members {
            match as_routes.get(member) {
                Some(as_route) => routes.extend(as_route),
                None => unrecorded_nums.push(*member),
            }
        }
        let mut result = Self {
            members: as_set.members.clone(),
            routes,
            unrecorded_nums,
            set_members: as_set.set_members.clone(),
        };
        result.clean_up();
        result
    }
}

pub fn clean_vec<T: Ord>(v: &mut Vec<T>) {
    v.sort();
    v.shrink_to_fit();
    v.dedup();
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AsProperty {
    /// Only imports from providers are specified.
    pub import_only_provider: bool,
    /// Only exports from providers are specified.
    pub export_only_provider: bool,
}

impl AsProperty {
    pub fn maybe_from_aut_num(num: u32, aut_num: &AutNum, db: &AsRelDb) -> Option<Self> {
        let import_only_provider = all_providers(&aut_num.imports, num, db);
        let export_only_provider = all_providers(&aut_num.exports, num, db);
        (import_only_provider || export_only_provider).then_some(Self {
            import_only_provider,
            export_only_provider,
        })
    }
}

fn all_providers(versions: &Versions, num: u32, db: &AsRelDb) -> bool {
    versions.entries_iter().all(|entry| {
        entry
            .mp_peerings
            .iter()
            .all(|peering| match &peering.mp_peering {
                Peering {
                    remote_as: AsExpr::Single(AsName::Num(they)),
                    remote_router: None,
                    local_router: None,
                } => db.get(*they, num) == Some(P2C),
                _ => false,
            })
    })
}

/// Cleaned RPSL intermediate representation ready for query.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QueryIr {
    pub aut_nums: HashMap<u32, AutNum>,
    pub as_sets: HashMap<String, AsSet>,
    pub route_sets: HashMap<String, RouteSet>,
    pub peering_sets: HashMap<String, PeeringSet>,
    pub filter_sets: HashMap<String, FilterSet>,
    /// Each value should always be sorted.
    pub as_routes: HashMap<u32, Vec<IpNet>>,
    /// Each value should always be sorted.
    pub as_set_routes: HashMap<String, AsSetRoute>,
    /// Special properties for some ASes.
    pub as_properties: HashMap<u32, AsProperty>,
}

impl QueryIr {
    /// Clean `ir` and use it to create a [`QueryIr`].
    pub fn from_ir(ir: Ir) -> Self {
        let Ir {
            aut_nums,
            as_sets,
            route_sets,
            peering_sets,
            filter_sets,
            mut as_routes,
        } = ir;
        as_routes.par_iter_mut().for_each(|(_, routes)| {
            routes.sort();
            routes.dedup();
            routes.shrink_to_fit();
        });
        let as_set_routes = as_sets
            .par_iter()
            .map(|(name, set)| (name.clone(), AsSetRoute::from_as_set(set, &as_routes)))
            .collect();
        let as_set_routes = flatten_as_set_routes(&as_set_routes);
        let as_set_routes = HashMap::from_iter(as_set_routes);
        let aut_nums = HashMap::from_iter(aut_nums);
        let as_sets = HashMap::from_iter(as_sets);
        let route_sets = HashMap::from_iter(route_sets);
        let peering_sets = HashMap::from_iter(peering_sets);
        let filter_sets = HashMap::from_iter(filter_sets);
        let as_routes = HashMap::from_iter(as_routes);
        Self {
            aut_nums,
            as_sets,
            route_sets,
            peering_sets,
            filter_sets,
            as_routes,
            as_set_routes,
            as_properties: HashMap::new(),
        }
    }

    /// Same as [`from_ir`](#method.from_ir),
    /// but with customer pseudo sets injected under names `c#{aut_num}`.
    pub fn from_ir_and_as_relationship(mut ir: Ir, db: &AsRelDb) -> Self {
        let pseudo_sets = make_customer_pseudo_set(db);
        ir.as_sets.extend(pseudo_sets);
        let as_properties = ir
            .aut_nums
            .iter()
            .filter_map(|(num, aut_num)| {
                AsProperty::maybe_from_aut_num(*num, aut_num, db).map(|a| (*num, a))
            })
            .collect();
        Self {
            as_properties,
            ..Self::from_ir(ir)
        }
    }
}

fn flatten_as_set_routes(
    as_set_routes: &BTreeMap<String, AsSetRoute>,
) -> BTreeMap<String, AsSetRoute> {
    let mut result = as_set_routes.clone();
    result.par_iter_mut().for_each(|(_, v)| {
        let members = mem::take(&mut v.set_members);
        for member in members {
            match as_set_routes.get(&member) {
                Some(as_set_route) => {
                    v.routes.extend(&as_set_route.routes);
                    v.unrecorded_nums.extend(&as_set_route.unrecorded_nums);
                    v.set_members.extend(as_set_route.set_members.clone());
                }
                None => v.set_members.push(member),
            }
        }
        v.clean_up();
    });
    result
}
