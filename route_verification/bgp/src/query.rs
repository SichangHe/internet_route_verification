use hashbrown::{HashMap, HashSet};

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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QueryAsSet {
    pub body: String,
    pub members: HashSet<u32>,
    pub unrecorded_members: Vec<String>,
    pub is_any: bool,
}

impl QueryAsSet {
    pub fn contains(&self, as_num: &u32) -> bool {
        self.is_any || self.members.contains(as_num)
    }

    pub fn clean_up(&mut self) {
        self.members.shrink_to_fit();
        clean_vec(&mut self.unrecorded_members);
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
    pub as_sets: HashMap<String, QueryAsSet>,
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
        let as_sets = flatten_as_sets(&as_sets);
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

fn flatten_as_set(
    query_as_set: &mut QueryAsSet,
    visited_sets: &mut HashSet<String>,
    set_members: &[String],
    as_sets: &BTreeMap<String, AsSet>,
) {
    for set_member in set_members {
        if !visited_sets.contains(set_member) {
            visited_sets.insert(set_member.to_string());
            if let Some(set) = as_sets.get(set_member) {
                query_as_set.members.extend(set.members.iter().copied());
            } else {
                query_as_set.unrecorded_members.push(set_member.to_string());
            }
        }
    }
}

pub fn flatten_as_sets(as_sets: &BTreeMap<String, AsSet>) -> HashMap<String, QueryAsSet> {
    as_sets
        .par_iter()
        .map(|(name, set)| {
            let AsSet {
                body,
                members,
                set_members,
                is_any,
            } = set;
            let mut query_as_set = QueryAsSet {
                body: body.clone(),
                members: HashSet::with_capacity(set_members.len() * 32 + members.len()),
                unrecorded_members: Vec::new(),
                is_any: *is_any,
            };
            query_as_set.members.extend(members.iter().copied());

            let mut visited = HashSet::with_capacity(set_members.len() * 8);
            visited.insert(name.to_string());
            flatten_as_set(&mut query_as_set, &mut visited, set_members, as_sets);

            query_as_set.clean_up();
            (name.to_owned(), query_as_set)
        })
        .collect()
}
