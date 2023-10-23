//! Evcxr usage example snippets.
#![allow(dead_code)]
#![allow(clippy::no_effect)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(clippy::type_complexity)]
#![allow(unused_mut)]

mod as_appeared_in_rules;
mod as_compatible_w_bgpq3;
mod as_route_stats;
mod as_stats;
mod as_w_single_rs_export;
mod filter_as;
mod route_stats;
mod specific_line;

use crate as route_verification;

/* Copy from the next line until the end of `use`.
If polars is needed.
```fish
set -gx RUSTFLAGS --cfg=fuzzing
```
before running Evcxr is also needed.

:opt 3
:dep anyhow
:dep dashmap
:dep route_verification = { path = "route_verification" }
:dep rayon
:dep itertools
:dep polars = { features = ["describe"] }
// */
use anyhow::Result;
use dashmap::DashMap;
use itertools::multiunzip;
use polars::prelude::*;
use rayon::prelude::*;
use route_verification::{
    as_rel::*,
    bgp::{stats::*, *},
    parse::*,
};
use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader, BufWriter},
    ops::Add,
    time::Instant,
};

fn read_parsed_rpsl() -> Result<()> {
    let start = Instant::now();
    let parsed = Ir::pal_read("parsed_all")?;
    println!("Read IR in {}ms.", start.elapsed().as_millis());
    let query = QueryIr::from_ir(parsed);

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

    let db = AsRelDb::load_bz("data/20230701.as-rel.bz2")?;
    let parsed = Ir::pal_read("parsed_all")?;
    let query: QueryIr = QueryIr::from_ir_and_as_relationship(parsed, &db);
    println!("{:#?}", query.aut_nums.iter().next());
    let mut bgp_lines: Vec<Line> = parse_mrt("data/mrts/rib.20230619.2200.bz2")?;

    Ok(())
}

/// Generate all the reports.
/// Copy this after running code from [`parse_bgp_lines`],
/// except maybe the `db` line.
fn gen_all_reports(query: QueryIr, mut bgp_lines: Vec<Line>) {
    let start = Instant::now();
    bgp_lines.par_iter_mut().for_each(|line| line.check(&query));
    println!("Used {}ms", start.elapsed().as_millis());
}

/// Benchmark for `match_ips`.
/// Copy this after running code from [`parse_bgp_lines`],
/// except maybe the `db` line.
fn benchmark_match_ips(query: QueryIr, bgp_lines: Vec<Line>) {
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
