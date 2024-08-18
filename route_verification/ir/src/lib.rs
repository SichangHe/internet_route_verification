use std::collections::BTreeMap;

use anyhow::{bail, Context, Error, Result};
use common_regex::{set::*, *};
use ipnet::IpNet;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

pub mod address_prefix;
pub mod aut_num;
pub mod aut_sys;
pub mod filter;
pub mod intermediate_repr;
pub mod mp_import;
pub mod peering;
pub mod router_expr;
pub mod set;
#[cfg(test)]
mod tests;

#[doc(inline)]
pub use {
    address_prefix::{match_ips, AddrPfxRange, RangeOperator},
    aut_num::AutNum,
    aut_sys::{is_as_set, is_pseudo_set, parse_as_name, parse_aut_num_name, AsName},
    filter::{is_any, is_filter_set, Filter},
    intermediate_repr::{merge_irs, Ir},
    mp_import::{Casts, Entry, Versions},
    peering::{is_peering_set, parse_single_as_expr, AsExpr, Peering, PeeringAction},
    router_expr::{parse_simple_router_expr, RouterExpr},
    set::{is_route_set_name, AsSet, FilterSet, PeeringSet, RouteSet, RouteSetMember},
    shared_struct::{action::*, community::Call, stats::Counts},
};
