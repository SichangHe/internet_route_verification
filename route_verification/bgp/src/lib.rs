//! Parse BGP routes and verify them against the RPSL [`Ir`].
use std::{collections::BTreeMap, mem};

use anyhow::Result;
use as_rel::{AsRelDb, Relationship::*};
use ipnet::IpNet;
use ir::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

pub mod cmp;
pub mod query;
pub mod report;
pub mod stats;
#[cfg(test)]
mod tests;
pub mod verbosity;
pub mod wrapper;

#[doc(inline)]
pub use {
    bgpmap::{self as map, AsPathEntry},
    cmp::Compare,
    query::{customer_set, AsProperty, QueryAsSet, QueryIr},
    report::{Report, ReportItem},
    verbosity::Verbosity,
    wrapper::{parse_mrt, Line},
};

use bgpmap::*;
use report::*;
use verbosity::*;
