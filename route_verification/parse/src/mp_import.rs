use ::lex::{mp_import, Counts};

use super::*;

pub fn parse_imports(imports: mp_import::Versions, counts: &mut Counts) -> Versions {
    let mp_import::Versions { any, ipv4, ipv6 } = imports;
    let any = parse_casts(any, counts);
    let ipv4 = parse_casts(ipv4, counts);
    let ipv6 = parse_casts(ipv6, counts);
    Versions { any, ipv4, ipv6 }
}

pub fn parse_casts(casts: mp_import::Casts, counts: &mut Counts) -> Casts {
    let mp_import::Casts {
        any,
        unicast,
        multicast,
    } = casts;
    let any = parse_entries(any, counts);
    let unicast = parse_entries(unicast, counts);
    let multicast = parse_entries(multicast, counts);
    Casts {
        any,
        unicast,
        multicast,
    }
}

pub fn parse_entries(entries: Vec<mp_import::Entry>, counts: &mut Counts) -> Vec<Entry> {
    entries
        .into_iter()
        .map(|e| parse_entry(e, counts))
        .collect()
}

pub fn parse_entry(entry: mp_import::Entry, counts: &mut Counts) -> Entry {
    let mp_import::Entry {
        mp_peerings,
        mp_filter,
    } = entry;
    let mp_peerings = parse_mp_peerings(mp_peerings);
    let mp_filter = parse_filter(mp_filter, counts);
    Entry {
        mp_peerings,
        mp_filter,
    }
}
