#![allow(dead_code)]
#![allow(clippy::no_effect)]
#![allow(unused_must_use)]

use super::*;
use crate as route_policy_cmp;

use route_policy_cmp::{bgp::*, parse::dump::Dump};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn read_parsed_rpsl() -> Result<()> {
    let parsed = Dump::pal_read("parsed_all")?;

    let bgp_file: Vec<String> = BufReader::new(File::open("data/bgp_routes_eg.txt")?)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    // Remove `;` in notebook.
    Compare::with_line_dump(&bgp_file[2])?.check(&parsed);

    Verbosity::Brief > Verbosity::ErrOnly;

    Ok(())
}

fn parse_bgp_lines() -> Result<()> {
    let parsed = Dump::pal_read("parsed_all")?;

    parsed.aut_nums.iter().next();

    let bgp_lines: Vec<Line> = parse_mrt("data/mrts/rib.20230619.2200.bz2")?;

    bgp_lines.first();

    bgp_lines[0].compare.check(&parsed);

    Ok(())
}
