use std::{collections::BTreeMap, mem};

use log::{debug, error};
use serde::{Deserialize, Serialize};

pub mod ast;
pub mod filter;
pub mod lines;
pub mod mp_import;
pub mod peering;
pub mod rpsl_object;
#[cfg(any(test, feature = "test_util"))]
pub mod test_util;
#[cfg(test)]
pub mod tests;

pub use {
    ast::Ast,
    filter::Filter,
    lines::{expressions, io_wrapper_lines, lines_continued, rpsl_objects, RPSLObject, RpslExpr},
    mp_import::{Casts, Entry, PeeringAction, Versions},
    peering::{AsExpr, ComplexAsExpr, Peering},
    rpsl_object::{AsOrRouteSet, AutNum, FilterSet, PeeringSet},
    shared_struct::{action::*, community::Call, stats::Counts},
};
