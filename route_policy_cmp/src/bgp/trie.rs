use std::net::{Ipv4Addr, Ipv6Addr};

use ip_network_table_deps_treebitmap::IpLookupTable;

// TODO: Efficient implementations for `Hash` and `Clone`.
#[derive(Default)]
pub struct IpTrie {
    pub v4: IpLookupTable<Ipv4Addr, ()>,
    pub v6: IpLookupTable<Ipv6Addr, ()>,
}

impl Eq for IpTrie {}

impl PartialEq for IpTrie {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for IpTrie {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IpTrie {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for (ours, theirs) in self.v4.iter().zip(other.v4.iter()) {
            match ours.cmp(&theirs) {
                std::cmp::Ordering::Equal => (),
                non_equal => return non_equal,
            }
        }
        for (ours, theirs) in self.v6.iter().zip(other.v6.iter()) {
            match ours.cmp(&theirs) {
                std::cmp::Ordering::Equal => (),
                non_equal => return non_equal,
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl std::hash::Hash for IpTrie {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.v4.iter().collect::<Vec<_>>().hash(state);
        self.v6.iter().collect::<Vec<_>>().hash(state);
    }
}

impl std::fmt::Debug for IpTrie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IpTrie")
            .field("v4", &self.v4.iter().collect::<Vec<_>>())
            .field("v4", &self.v4.iter().collect::<Vec<_>>())
            .finish()
    }
}

impl Clone for IpTrie {
    fn clone(&self) -> Self {
        let mut v4 = IpLookupTable::with_capacity(self.v4.len());
        self.v4
            .iter()
            .for_each(|(ip, masklen, _)| _ = v4.insert(ip, masklen, ()));
        let mut v6 = IpLookupTable::with_capacity(self.v6.len());
        self.v6
            .iter()
            .for_each(|(ip, masklen, _)| _ = v6.insert(ip, masklen, ()));
        Self { v4, v6 }
    }
}
