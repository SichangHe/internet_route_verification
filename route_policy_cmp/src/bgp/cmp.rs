use std::convert::identity;

use anyhow::{Context, Result};
use ipnet::IpNet;
use log::debug;

use crate::parse::{lex::Dump, mp_import::Versions};

use super::map::{parse_table_dump, AsPathEntry};

pub fn compare_line_w_dump(line: &str, dump: &Dump) -> Result<Vec<Report>> {
    let (prefix, as_path, _, communities) = parse_table_dump(line)?;
    // Iterate the pairs in `as_path` from right to left, with overlaps.
    let pairs = as_path.iter().rev().zip(as_path.iter().rev().skip(1));
    let reports = pairs
        .map(|(from, to)| {
            if let (AsPathEntry::Seq(from), AsPathEntry::Seq(to)) = (from, to) {
                compare_pair_w_dump(*from, *to, dump, prefix, &communities)
                    .map_or_else(|e| Report::Skipped(format!("{e:#}")), identity)
            } else {
                let err = "Skipping BGP pair {from}, {to} with set.".into();
                debug!("{err}");
                Report::Skipped(err)
            }
        })
        .collect();
    Ok(reports)
}

pub fn compare_pair_w_dump(
    from: usize,
    to: usize,
    dump: &Dump,
    prefix: IpNet,
    communities: &[&str],
) -> Result<Report> {
    let from_an = dump
        .aut_nums
        .get(&from)
        .context("{from} is not a recorded AutNum")?;
    let to_an = dump
        .aut_nums
        .get(&to)
        .context("{to} is not a recorded AutNum")?;
    check_complient(&from_an.exports, to, prefix, communities)?;
    check_complient(&to_an.exports, from, prefix, communities)?;
    todo!()
}

pub fn check_complient(
    policy: &Versions,
    accept_num: usize,
    prefix: IpNet,
    communities: &[&str],
) -> Result<()> {
    todo!()
}

pub enum Report {
    Skipped(String),
}
