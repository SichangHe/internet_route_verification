use anyhow::Result;
use ipnet::IpNet;

use crate::parse::{lex::Dump, mp_import::Versions};

use super::map::{parse_table_dump, AsPathEntry};

pub fn compare_line_w_dump(line: &str, dump: &Dump) -> Result<Vec<Report>> {
    let (prefix, as_path, _, communities) = parse_table_dump(line)?;
    // Iterate the pairs in `as_path` from right to left, with overlaps.
    let pairs = as_path.iter().rev().zip(as_path.iter().rev().skip(1));
    let reports = pairs
        .flat_map(|(from, to)| {
            if let (AsPathEntry::Seq(from), AsPathEntry::Seq(to)) = (from, to) {
                compare_pair_w_dump(*from, *to, dump, prefix, &communities)
            } else {
                vec![Report::Skipped(format!(
                    "Skipping BGP pair {from}, {to} with set."
                ))]
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
) -> Vec<Report> {
    let mut from_report = match dump.aut_nums.get(&from) {
        Some(from_an) => check_complient(&from_an.exports, to, prefix, communities),
        None => vec![Report::Skipped(format!("{from} is not a recorded AutNum"))],
    };
    let to_report = match dump.aut_nums.get(&to) {
        Some(to_an) => check_complient(&to_an.imports, from, prefix, communities),
        None => vec![Report::Skipped(format!("{to} is not a recorded AutNum"))],
    };
    from_report.extend(to_report);
    from_report
}

pub fn check_complient(
    policy: &Versions,
    accept_num: usize,
    prefix: IpNet,
    communities: &[&str],
) -> Vec<Report> {
    todo!()
}

pub enum Report {
    Skipped(String),
}
