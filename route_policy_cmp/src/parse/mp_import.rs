use serde::{Deserialize, Serialize};

use crate::lex::mp_import;

use super::{
    filter::{parse_filter, Filter},
    peering::{parse_mp_peerings, PeeringAction},
};

pub fn parse_imports(imports: mp_import::Versions) -> Versions {
    let mp_import::Versions { any, ipv4, ipv6 } = imports;
    let any = parse_casts(any);
    let ipv4 = parse_casts(ipv4);
    let ipv6 = parse_casts(ipv6);
    Versions { any, ipv4, ipv6 }
}

pub fn parse_casts(casts: mp_import::Casts) -> Casts {
    let mp_import::Casts {
        any,
        unicast,
        multicast,
    } = casts;
    let any = parse_entries(any);
    let unicast = parse_entries(unicast);
    let multicast = parse_entries(multicast);
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
    let mp_filter = parse_filter(mp_filter, &mp_peerings);
    Entry {
        mp_peerings,
        mp_filter,
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Versions {
    pub any: Casts,
    pub ipv4: Casts,
    pub ipv6: Casts,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Casts {
    pub any: Vec<Entry>,
    pub unicast: Vec<Entry>,
    pub multicast: Vec<Entry>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Entry {
    pub mp_peerings: Vec<PeeringAction>,
    pub mp_filter: Filter,
}
