//! This is originally copied from
//! <https://github.com/cunha/measurements/blob/9a14123b4c9d47297fa4c284ff8dd0834ba73936/bgp/bgpmap/src/lib.rs>.
use std::{fmt::Display, net::IpAddr, str::FromStr};

use anyhow::Result;
use ipnet::IpNet;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CollectorPeer {
    asn: u32,
    ip: IpAddr,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AsPathEntry {
    Seq(u32),
    Set(Vec<u32>),
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
    type Item = u32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            AsPathEntry::Seq(asn) => vec![asn].into_iter(),
            AsPathEntry::Set(sset) => sset.into_iter(),
        }
    }
}

impl FromStr for AsPathEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\{(\d+(?:,\d+)*)\}").unwrap();
        }
        // let ee: Regex = Regex::new(r"\{(\d+(?:,\d+)*)\}").unwrap();
        let asn = s.parse::<u32>();
        if let Ok(n) = asn {
            return Ok(AsPathEntry::Seq(n));
        }
        let caps = RE.captures(s).ok_or("as-path-entry-no-match")?;
        // Regex should guarantee the unwraps never fail:
        let asset: Vec<u32> = caps
            .get(1)
            .unwrap()
            .as_str()
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        Ok(AsPathEntry::Set(asset))
    }
}

fn parse_bgpdump_table_dump_v2(
    line: &str,
) -> Result<(IpNet, Vec<AsPathEntry>, CollectorPeer), String> {
    // TABLE_DUMP2|1619481601|B|94.156.252.18|34224|6.132.0.0/14|34224 6939 8003|IGP|94.156.252.18|0|0|34224:333 34224:334 34224:2040|NAG|||
    // TABLE_DUMP2|1661040000|B|94.177.122.251|58057|2001:410::/32|58057 174 1299 1299 1299 2603 2603 2603 6509 {271,7860,8111,10972,53904}|IGP|::ffff:94.177.122.251|0|0|174:21100 58057:65010 174:22005|AG|6509 205.189.32.101|
    assert!(line.starts_with("TABLE_DUMP2"));
    let fields: Vec<&str> = line.split('|').collect();
    let vp = CollectorPeer {
        asn: fields[4]
            .parse::<u32>()
            .map_err(|_| String::from("bad-vp-asn"))?,
        ip: fields[3]
            .parse::<IpAddr>()
            .map_err(|_| String::from("bad-vp-ip"))?,
    };
    let prefix: IpNet = fields[5]
        .parse::<IpNet>()
        .map_err(|_| String::from("bad-prefix"))?;

    let aspath: Vec<AsPathEntry> = fields[6]
        .split(' ')
        .map(|e| e.parse::<AsPathEntry>())
        .collect::<Result<Vec<AsPathEntry>, String>>()?;

    Ok((prefix, aspath, vp))
}
