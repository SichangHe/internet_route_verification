use anyhow::Result;
use ipnet::IpNet;

use crate::parse::{
    lex::Dump,
    mp_import::{Casts, Entry, Versions},
};

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
        Some(from_an) => check_compliant(&from_an.exports, to, prefix, communities),
        None => vec![Report::Skipped(format!("{from} is not a recorded AutNum"))],
    };
    let to_report = match dump.aut_nums.get(&to) {
        Some(to_an) => check_compliant(&to_an.imports, from, prefix, communities),
        None => vec![Report::Skipped(format!("{to} is not a recorded AutNum"))],
    };
    from_report.extend(to_report);
    from_report
}

pub fn check_compliant(
    policy: &Versions,
    accept_num: usize,
    prefix: IpNet,
    communities: &[&str],
) -> Vec<Report> {
    let mut reports = Vec::with_capacity(2);
    if let Some(specific_report) = match prefix {
        IpNet::V4(_) => check_casts_compliant(&policy.ipv4, accept_num, prefix, communities),
        IpNet::V6(_) => check_casts_compliant(&policy.ipv6, accept_num, prefix, communities),
    } {
        reports.push(specific_report);
    }
    if let Some(general_report) =
        check_casts_compliant(&policy.any, accept_num, prefix, communities)
    {
        reports.push(general_report);
    }
    reports
}

pub fn check_casts_compliant(
    casts: &Casts,
    accept_num: usize,
    prefix: IpNet,
    communities: &[&str],
) -> Option<Report> {
    // TODO: How do we know the casts?
    for entry in [&casts.multicast, &casts.unicast, &casts.any]
        .into_iter()
        .flatten()
    {
        if let Some(report) = check_entry_compliant(entry, accept_num, prefix, communities) {
            return Some(report);
        }
    }
    None
}

pub fn check_entry_compliant(
    entry: &Entry,
    accept_num: usize,
    prefix: IpNet,
    communities: &[&str],
) -> Option<Report> {
    todo!()
}

pub enum Report {
    Skipped(String),
}
