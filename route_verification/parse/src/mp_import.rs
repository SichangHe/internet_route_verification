use ::lex::mp_import;

use super::*;

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
