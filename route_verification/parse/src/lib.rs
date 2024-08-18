use std::collections::BTreeMap;

use anyhow::{bail, Context, Result};

use ipnet::IpNet;
use ir::*;
use log::error;
use rayon::prelude::*;

use shared_struct::{action::Actions, stats::Counts};

pub mod action;
pub mod filter;
pub mod lex;
pub mod mp_import;
pub mod peering;
pub mod router_expr;
#[cfg(test)]
mod tests;

#[doc(inline)]
pub use {
    self::lex::parse_lexed,
    action::parse_actions,
    filter::parse_filter,
    mp_import::parse_imports,
    peering::{parse_mp_peering, parse_mp_peerings},
    router_expr::parse_router_expr,
};
