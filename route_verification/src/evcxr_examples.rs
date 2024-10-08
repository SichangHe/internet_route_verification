//! Evcxr usage example snippets.
#![allow(dead_code)]
#![allow(clippy::no_effect)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(clippy::type_complexity)]
#![allow(unused_mut)]
#![allow(clippy::ptr_arg)]

mod as_appeared_in_rules;
mod as_compatible_w_bgpq3;
mod as_neighbors_vs_rules;
mod as_path_scan_all_ribs;
mod as_set_graphing;
mod as_stats;
mod as_w_routes_wo_aut_num;
mod as_w_single_rs_export;
mod bgpq3_compatible_rules;
mod collect_source;
mod community_filter;
mod count_asn_in_peering;
mod count_path_sets;
mod count_router_info;
mod filter_as;
mod filter_percentages;
mod last_modified;
mod object_referred_in_rules;
mod route_stats;
mod specific_line;
mod transit_as_rules;

use crate as route_verification;

/* Copy from the next line until the end of `use`.
If polars is needed.
```fish
export RUSTFLAGS=--cfg=fuzzing
```
before running Evcxr is also needed.

:opt 3
:dep anyhow
:dep dashmap
:dep hashbrown
:dep route_verification = { path = "route_verification" }
:dep rayon
:dep itertools
:dep serde_json
// */
use anyhow::Result;
use dashmap::{DashMap, DashSet};
use hashbrown::{HashMap, HashSet};
use itertools::multiunzip;
use rayon::prelude::*;
use route_verification::as_rel::*;
use route_verification::bgp::stats::*;
use route_verification::bgp::*;
use route_verification::fs::open_file_w_correct_encoding;
use route_verification::ir::*;
use route_verification::irr::split_commas;
use route_verification::lex::{
    expressions, io_wrapper_lines, lines_continued, rpsl_objects, RpslExpr,
};
use std::{
    env,
    fs::{read_dir, read_to_string, File},
    io::{prelude::*, BufReader, BufWriter},
    ops::Add,
    time::Instant,
};

// If Polars is needed:
/*
:dep polars = { features = ["describe"] }
// */
use polars::prelude::*;
// */
fn read_parsed_rpsl() {
    let start = Instant::now();
    let parsed = Ir::pal_read("parsed_all").unwrap();
    println!("Read IR in {}ms.", start.elapsed().as_millis());
    let query = QueryIr::from_ir(parsed);

    let start = Instant::now();
    let bgp_file: Vec<String> = BufReader::new(File::open("data/bgp_routes_eg.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .collect();
    println!("Read BGP file in {}ms.", start.elapsed().as_millis());

    Compare::with_line_dump(&bgp_file[2]).unwrap().check(&query)
    // Exclude `;` when copying.
    ;
}

fn parse_bgp_lines() {
    // <https://pola-rs.github.io/polars/polars/index.html#config-with-env-vars>
    env::set_var("POLARS_FMT_MAX_COLS", "32");
    env::set_var("POLARS_TABLE_WIDTH", "160");

    let db = AsRelDb::load_bz("data/20230701.as-rel.bz2").unwrap();
    let ir = Ir::pal_read("parsed_all").unwrap();
    println!(
        "{}",
        serde_json::to_string(ir.aut_nums.get(&33549).unwrap()).unwrap()
    );
    let query: QueryIr = QueryIr::from_ir_and_as_relationship(ir.clone(), &db);
    println!("{:#?}", query.aut_nums.iter().next());
    let mut bgp_lines: Vec<Line> = parse_mrt("data/mrts/rib.20230619.2200.bz2").unwrap();
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
