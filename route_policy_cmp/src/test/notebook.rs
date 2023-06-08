use super::*;
use crate as route_policy_cmp;

use route_policy_cmp::{bgp::cmp::compare_line_w_dump, parse::lex::Dump, serde::from_reader};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

#[allow(dead_code)]
#[allow(unused_must_use)]
fn example() -> Result<()> {
    let parsed: Dump = from_reader(File::open("parsed.json")?)?;

    let bgp_file: Vec<String> = BufReader::new(File::open("data/bgp_routes_eg.txt")?)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    // Remove `;` in notebook.
    compare_line_w_dump(&bgp_file[2], &parsed);

    Ok(())
}
