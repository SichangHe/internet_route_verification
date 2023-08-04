use std::{collections::BTreeMap, mem};

use anyhow::Result;
use as_rel::{AsRelDb, Relationship::*};
use bloom::BloomHashSet;
use ipnet::IpNet;
use parse::*;
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

pub use {
    cmp::Compare,
    map::{self, AsPathEntry},
    query::{customer_set, AsProperty, AsSetRoute, QueryDump},
    report::{MatchProblem, Report, ReportItem, SkipReason},
    stats::{AsPairStats, AsStats, UpDownHillStats},
    verbosity::Verbosity,
    wrapper::{parse_mrt, Line},
};

use map::*;
use report::*;
use verbosity::*;
