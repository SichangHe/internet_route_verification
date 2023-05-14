use serde::{Deserialize, Serialize};

use super::mp_import::Versions;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AutNum {
    pub body: String,
    pub errors: Vec<String>,
    pub imports: Versions,
    pub exports: Versions,
}
