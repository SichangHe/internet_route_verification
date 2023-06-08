use serde::{Deserialize, Serialize};

use super::mp_import::Versions;

#[derive(Clone, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AutNum {
    pub body: String,
    pub imports: Versions,
    pub exports: Versions,
}

impl std::fmt::Debug for AutNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut r = f.debug_struct("AutNum");
        r.field("body", &self.body);
        for (name, field) in [("imports", &self.imports), ("exports", &self.exports)] {
            if !field.is_default() {
                r.field(name, field);
            }
        }
        r.finish()
    }
}
