//! This is originally copied from
//! <https://github.com/cunha/measurements/blob/9a14123b4c9d47297fa4c284ff8dd0834ba73936/bgp/bgpmap/src/lib.rs>.
use std::{fmt::Display, net::IpAddr, str::FromStr};

use anyhow::{bail, Context, Result};
use ipnet::IpNet;
use lazy_regex::regex_captures;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CollectorPeer {
    asn: usize,
    ip: IpAddr,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AsPathEntry {
    Seq(usize),
    Set(Vec<usize>),
}

impl Display for AsPathEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsPathEntry::Seq(asn) => write!(f, "{asn}"),
            AsPathEntry::Set(sset) => write!(
                f,
                "{}",
                sset.iter()
                    .map(|asn| format!("{asn}"))
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        }
    }
}

impl IntoIterator for AsPathEntry {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            AsPathEntry::Seq(asn) => vec![asn].into_iter(),
            AsPathEntry::Set(sset) => sset.into_iter(),
        }
    }
}

impl FromStr for AsPathEntry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let asn = s.parse();
        if let Ok(n) = asn {
            return Ok(AsPathEntry::Seq(n));
        }
        let (_, cap) =
            regex_captures!(r"\{(\d+(?:,\d+)*)\}", s).context("as-path-entry-no-match")?;
        // Regex should guarantee the unwraps never fail:
        let asset = cap.split(',').map(|n| n.parse().unwrap()).collect();
        Ok(AsPathEntry::Set(asset))
    }
}

/// Return (IP prefix, AS-path, BGP collector).
pub fn parse_bgpdump_table_dump_v2(line: &str) -> Result<(IpNet, Vec<AsPathEntry>, CollectorPeer)> {
    // TABLE_DUMP2|1619481601|B|94.156.252.18|34224|6.132.0.0/14|34224 6939 8003|IGP|94.156.252.18|0|0|34224:333 34224:334 34224:2040|NAG|||
    // TABLE_DUMP2|1661040000|B|94.177.122.251|58057|2001:410::/32|58057 174 1299 1299 1299 2603 2603 2603 6509 {271,7860,8111,10972,53904}|IGP|::ffff:94.177.122.251|0|0|174:21100 58057:65010 174:22005|AG|6509 205.189.32.101|
    if !(line.starts_with("TABLE_DUMP2")) {
        bail!("{line} does not start with TABLE_DUMP2");
    }
    let fields: Vec<_> = line.split('|').collect();
    if fields.len() < 7 {
        bail!("{line} breaks down to less than 7 fields");
    }
    let vp = CollectorPeer {
        asn: fields[4].parse().context("bad-vp-asn")?,
        ip: fields[3].parse().context("bad-vp-ip")?,
    };
    let prefix = fields[5].parse().context("bad-prefix")?;

    let aspath: Vec<AsPathEntry> = fields[6]
        .split(' ')
        .map(|e| e.parse())
        .collect::<Result<_>>()?;

    Ok((prefix, aspath, vp))
}
