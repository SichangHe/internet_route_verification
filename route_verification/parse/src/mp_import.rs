use ::lex::{mp_import, Counts};
use itertools::chain;

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
    let mp_filter = parse_filter(mp_filter, &mp_peerings, counts);
    Entry {
        mp_peerings,
        mp_filter,
    }
}

#[derive(Clone, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(default)]
pub struct Versions {
    #[serde(skip_serializing_if = "Casts::is_default")]
    pub any: Casts,
    #[serde(skip_serializing_if = "Casts::is_default")]
    pub ipv4: Casts,
    #[serde(skip_serializing_if = "Casts::is_default")]
    pub ipv6: Casts,
}

impl Versions {
    /// Iterator of all the entries in these versions.
    pub fn entries_iter(&self) -> impl Iterator<Item = &Entry> {
        chain!(
            self.any.entries_iter(),
            self.ipv4.entries_iter(),
            self.ipv6.entries_iter()
        )
    }

    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl std::fmt::Debug for Versions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut r = f.debug_struct("Versions");
        for (name, field) in [
            ("any", &self.any),
            ("ipv4", &self.ipv4),
            ("ipv6", &self.ipv6),
        ] {
            if !field.is_default() {
                r.field(name, field);
            }
        }
        r.finish()
    }
}

#[derive(Clone, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(default)]
pub struct Casts {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub any: Vec<Entry>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub unicast: Vec<Entry>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub multicast: Vec<Entry>,
}

impl std::fmt::Debug for Casts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut r = f.debug_struct("Casts");
        for (name, field) in [
            ("any", &self.any),
            ("unicast", &self.unicast),
            ("multicast", &self.multicast),
        ] {
            if !field.is_empty() {
                r.field(name, field);
            }
        }
        r.finish()
    }
}

impl Casts {
    /// Iterator of all the entries in these casts.
    pub fn entries_iter(&self) -> impl Iterator<Item = &Entry> {
        chain!(self.any.iter(), self.unicast.iter(), self.multicast.iter())
    }

    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Entry {
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mp_peerings: Vec<PeeringAction>,
    pub mp_filter: Filter,
}
