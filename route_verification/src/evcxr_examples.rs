//! Evcxr usage example snippets.
#![allow(dead_code)]
#![allow(clippy::no_effect)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(clippy::type_complexity)]
#![allow(unused_mut)]

mod as_stats;
mod as_route_stats;

use super::*;
use crate as route_verification;

/* Copy from the next line until the end of `use`.
:opt 3
:dep dashmap
:dep route_verification = { path = "route_verification" }
:dep rayon
:dep polars = { features = ["describe"] }
:dep itertools
// */
use dashmap::DashMap;
use itertools::multiunzip;
use polars::prelude::*;
use rayon::prelude::*;
use route_verification::{as_rel::*, bgp::*, parse::*};
use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    ops::Add,
    time::Instant,
};

fn read_parsed_rpsl() -> Result<()> {
    let start = Instant::now();
    let parsed = Dump::pal_read("parsed_all")?;
    println!("Read dump in {}ms.", start.elapsed().as_millis());
    let query = QueryDump::from_dump(parsed);

    let start = Instant::now();
    let bgp_file: Vec<String> = BufReader::new(File::open("data/bgp_routes_eg.txt")?)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    println!("Read BGP file in {}ms.", start.elapsed().as_millis());

    Compare::with_line_dump(&bgp_file[2])?.check(&query)
    // Exclude `;` when copying.
    ;

    Ok(())
}

fn parse_bgp_lines() -> Result<()> {
    // <https://pola-rs.github.io/polars/polars/index.html#config-with-env-vars>
    env::set_var("POLARS_FMT_MAX_COLS", "32");
    env::set_var("POLARS_TABLE_WIDTH", "160");

    let parsed = Dump::pal_read("parsed_all")?;
    let query: QueryDump = QueryDump::from_dump(parsed);
    println!("{:#?}", query.aut_nums.iter().next());
    let mut bgp_lines: Vec<Line> = parse_mrt("data/mrts/rib.20230619.2200.bz2")?;
    let db = AsRelDb::load_bz("data/20230701.as-rel.bz2")?;

    Ok(())
}

/// Generate all the reports.
/// Copy this after running code from [`parse_bgp_lines`],
/// except maybe the `db` line.
fn gen_all_reports(query: QueryDump, mut bgp_lines: Vec<Line>) {
    let start = Instant::now();
    bgp_lines.par_iter_mut().for_each(|line| line.check(&query));
    println!("Used {}ms", start.elapsed().as_millis());
}

/// Benchmark for `match_ips`.
/// Copy this after running code from [`parse_bgp_lines`],
/// except maybe the `db` line.
fn benchmark_match_ips(query: QueryDump, bgp_lines: Vec<Line>) {
    const SIZE: usize = 0x10000;
    let start = Instant::now();
    let n_error: usize = bgp_lines[..SIZE]
        .par_iter()
        .map(|line| {
            if line.compare.check(&query).is_empty() {
                0
            } else {
                1
            }
        })
        .sum();
    println!(
        "Found {n_error} in {SIZE} routes in {}ms",
        start.elapsed().as_millis()
    );
}
