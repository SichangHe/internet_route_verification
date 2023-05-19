use serde::{Deserialize, Serialize};

use super::aut_sys::AsName;

/// <https://www.rfc-editor.org/rfc/rfc2622#section-5.1>
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AsSet {
    pub body: String,
    pub members: Vec<AsName>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RouteSet {}
