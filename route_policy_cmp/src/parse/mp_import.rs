use serde::{Deserialize, Serialize};

use crate::lex::mp_import;

use super::{action::Actions, filter::Filter, peering::Peering};

pub fn parse_imports(imports: mp_import::Versions) -> Versions {
    let mp_import::Versions { any, ipv4, ipv6 } = imports;
    let any = any.map(parse_casts);
    let ipv4 = ipv4.map(parse_casts);
    let ipv6 = ipv6.map(parse_casts);
    Versions { any, ipv4, ipv6 }
}

pub fn parse_casts(casts: mp_import::Casts) -> Casts {
    let mp_import::Casts {
        any,
        unicast,
        multicast,
    } = casts;
    let any = any.map(parse_entries);
    let unicast = unicast.map(parse_entries);
    let multicast = multicast.map(parse_entries);
    Casts {
        any,
        unicast,
        multicast,
    }
}

pub fn parse_entries(entries: Vec<mp_import::Entry>) -> Vec<Entry> {
    todo!("{entries:?}")
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Versions {
    pub any: Option<Casts>,
    pub ipv4: Option<Casts>,
    pub ipv6: Option<Casts>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Casts {
    pub any: Option<Vec<Entry>>,
    pub unicast: Option<Vec<Entry>>,
    pub multicast: Option<Vec<Entry>>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Entry {
    pub mp_peerings: Vec<PeeringAction>,
    pub mp_filter: Filter,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PeeringAction {
    pub mp_peering: Peering,
    pub actions: Option<Actions>,
}
