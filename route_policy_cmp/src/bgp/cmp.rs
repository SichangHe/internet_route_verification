use anyhow::Result;
use ipnet::IpNet;
use log::debug;

use crate::parse::lex::Dump;

use super::map::{parse_table_dump, AsPathEntry};

pub fn compare_line_w_dump(line: &str, dump: &Dump) -> Result<()> {
    let (prefix, as_path, _, communities) = parse_table_dump(line)?;
    // Iterate the pairs in `as_path` from right to left, with overlaps.
    for (from, to) in as_path.iter().rev().zip(as_path.iter().rev().skip(1)) {
        if let (AsPathEntry::Seq(from), AsPathEntry::Seq(to)) = (from, to) {
            compare_pair_w_dump(*from, *to, dump, prefix, &communities)?;
        } else {
            debug!("Skipping BGP pair {from}, {to} with set.")
        }
    }
    Ok(())
}

pub fn compare_pair_w_dump(
    from: usize,
    to: usize,
    dump: &Dump,
    prefix: IpNet,
    communities: &[&str],
) -> Result<()> {
    todo!()
}
