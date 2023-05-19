use lazy_regex::regex_is_match;
use serde::{Deserialize, Serialize};

use super::aut_sys::AsName;

/// <https://www.rfc-editor.org/rfc/rfc2622#section-5.1>
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AsSet {
    pub body: String,
    pub members: Vec<AsName>,
}

pub fn is_route_set_name(attr: &str) -> bool {
    regex_is_match!(r"^(AS\d+:)?rs-\S+$"i, attr)
}

/// <https://www.rfc-editor.org/rfc/rfc2622#section-5.2>
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RouteSet {
    pub body: String,
    /// List of `<address-prefix-range>` or `<route-set-name>` or
    /// `<route-set-name><range-operator>`.
    // TODO: Parse them.
    pub members: Vec<String>,
}
