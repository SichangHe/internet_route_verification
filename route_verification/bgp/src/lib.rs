use std::{collections::BTreeMap, mem};

use anyhow::Result;
use bloom::BloomHashSet;
use ipnet::IpNet;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

pub mod cmp;
pub mod filter;
pub mod map;
pub mod peering;
pub mod query;
pub mod report;
pub mod stats;
#[cfg(test)]
mod tests;
pub mod verbosity;
pub mod wrapper;

pub use {
    cmp::Compare,
    query::{AsSetRoute, QueryDump},
    report::{MatchProblem, Report, ReportItem, SkipReason},
    stats::AsStats,
    verbosity::Verbosity,
    wrapper::{parse_mrt, Line},
};

use filter::CheckFilter;
use map::*;
use peering::CheckPeering;
use report::*;
use verbosity::*;
