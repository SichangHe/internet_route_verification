use itertools::chain;

use super::*;

/// Clustered by their IP versions, a collection of rule entries.
#[derive(Clone, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(default)]
pub struct Versions {
    #[serde(skip_serializing_if = "Casts::is_empty")]
    pub any: Casts,
    #[serde(skip_serializing_if = "Casts::is_empty")]
    pub ipv4: Casts,
    #[serde(skip_serializing_if = "Casts::is_empty")]
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

    pub fn len(&self) -> usize {
        self.any.len() + self.ipv4.len() + self.ipv6.len()
    }

    pub fn is_empty(&self) -> bool {
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
            if !field.is_empty() {
                r.field(name, field);
            }
        }
        r.finish()
    }
}

/// Clustered by their cast types (unicast vs. multicast),
/// a collection of rule entries.
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

    pub fn len(&self) -> usize {
        self.any.len() + self.unicast.len() + self.multicast.len()
    }

    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

/// Representation of an RPSL rule entry, consisting of one or
/// more `<mp-peering> [<actions>]` and a `<mp-filter>`.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Entry {
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mp_peerings: Vec<PeeringAction>,
    pub mp_filter: Filter,
}
