use serde::{Deserialize, Serialize};

use super::{action::Actions, filter::Filter, peering::Peering};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(default)]
pub struct Versions {
    pub any: Casts,
    pub ipv4: Casts,
    pub ipv6: Casts,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(default)]
pub struct Casts {
    pub any: Vec<Entry>,
    pub unicast: Vec<Entry>,
    pub multicast: Vec<Entry>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Entry {
    pub mp_peerings: Vec<PeeringAction>,
    #[serde(default)]
    pub mp_filter: Filter,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PeeringAction {
    pub mp_peering: Peering,
    #[serde(default)]
    pub actions: Actions,
}
