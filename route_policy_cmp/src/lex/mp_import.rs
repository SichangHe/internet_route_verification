use serde::{Deserialize, Serialize};

use super::{action::Actions, filter::Filter, peering::Peering};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Versions {
    pub any: Option<Casts>,
    pub ipv4: Option<Casts>,
    pub ipv6: Option<Casts>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Casts {
    pub any: Option<Vec<Entry>>,
    pub unicast: Option<Vec<Entry>>,
    pub multicast: Option<Vec<Entry>>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Entry {
    pub mp_peerings: Vec<PeeringAction>,
    pub mp_filter: Filter,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PeeringAction {
    pub mp_peering: Peering,
    pub actions: Option<Actions>,
}
