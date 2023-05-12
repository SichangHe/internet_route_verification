use serde::{Deserialize, Serialize};

use super::mp_import::Versions;

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AutNum {
    pub name: String,
    pub body: String,
    pub imports: Versions,
    pub exports: Versions,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AsOrRouteSet {
    pub name: String,
    pub body: String,
    pub members: Vec<String>,
}
