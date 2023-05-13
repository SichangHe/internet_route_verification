use serde::{Deserialize, Serialize};

use crate::lex::mp_import;

use super::{
    filter::{parse_filter, Filter},
    peering::{parse_mp_peerings, PeeringAction},
};

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
    entries.into_iter().map(parse_entry).collect()
}

pub fn parse_entry(entry: mp_import::Entry) -> Entry {
    let mp_import::Entry {
        mp_peerings,
        mp_filter,
    } = entry;
    let mp_peerings = parse_mp_peerings(mp_peerings);
    let mp_filter = parse_filter(mp_filter);
    Entry {
        mp_peerings,
        mp_filter,
    }
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
