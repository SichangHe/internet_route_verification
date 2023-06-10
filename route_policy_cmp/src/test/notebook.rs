use super::*;
use crate as route_policy_cmp;

use route_policy_cmp::{bgp::cmp::Compare, parse::dump::Dump};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

#[allow(dead_code)]
#[allow(unused_must_use)]
fn example() -> Result<()> {
    let parsed = Dump::pal_read("parsed")?;

    let bgp_file: Vec<String> = BufReader::new(File::open("data/bgp_routes_eg.txt")?)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    // Remove `;` in notebook.
    Compare::with_line_dump(&bgp_file[2], &parsed)?.check();

    Ok(())
}
