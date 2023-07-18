//! Evcxr usage example snippets.
#![allow(dead_code)]
#![allow(clippy::no_effect)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(clippy::type_complexity)]
#![allow(unused_mut)]

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
    let parsed = Dump::pal_read("parsed_all")?;
    let query: QueryDump = QueryDump::from_dump(parsed);
    println!("{:#?}", query.aut_nums.iter().next());
    let mut bgp_lines: Vec<Line> = parse_mrt("data/mrts/rib.20230619.2200.bz2")?;
    let db = AsRelDb::load_bz("data/20230701.as-rel.bz2")?;

    Ok(())
}

/// Generate statistics for up/downhill.
/// Copy this after running code from [`parse_bgp_lines`].
fn gen_up_down_hill_stats(query: QueryDump, mut bgp_lines: Vec<Line>, db: AsRelDb) -> Result<()> {
    let start = Instant::now();
    let up_down_hill_stats: UpDownHillStats = bgp_lines
        .par_iter_mut()
        .map(|l| l.compare.up_down_hill_stats(&query, &db))
        .reduce(UpDownHillStats::default, Add::add);
    let total = up_down_hill_stats.sum();
    println!(
        "Generated stats of {total} reports in {}ms.",
        start.elapsed().as_millis()
    );

    let mut up_down_hill_df = DataFrame::new(vec![
        Series::new(
            "quality",
            vec![
                "good", "good", "good", "good", "good", "good", "good", "good", "neutral",
                "neutral", "neutral", "neutral", "neutral", "neutral", "neutral", "neutral", "bad",
                "bad", "bad", "bad", "bad", "bad", "bad", "bad",
            ],
        ),
        Series::new(
            "hill",
            vec![
                "up", "down", "peer", "other", "up", "down", "peer", "other", "up", "down", "peer",
                "other", "up", "down", "peer", "other", "up", "down", "peer", "other", "up",
                "down", "peer", "other",
            ],
        ),
        Series::new(
            "port",
            vec![
                "import", "import", "import", "import", "export", "export", "export", "export",
                "import", "import", "import", "import", "export", "export", "export", "export",
                "import", "import", "import", "import", "export", "export", "export", "export",
            ],
        ),
        Series::new(
            "value",
            vec![
                up_down_hill_stats.good_up_import,
                up_down_hill_stats.good_down_import,
                up_down_hill_stats.good_peer_import,
                up_down_hill_stats.good_other_import,
                up_down_hill_stats.good_up_export,
                up_down_hill_stats.good_down_export,
                up_down_hill_stats.good_peer_export,
                up_down_hill_stats.good_other_export,
                up_down_hill_stats.neutral_up_import,
                up_down_hill_stats.neutral_down_import,
                up_down_hill_stats.neutral_peer_import,
                up_down_hill_stats.neutral_other_import,
                up_down_hill_stats.neutral_up_export,
                up_down_hill_stats.neutral_down_export,
                up_down_hill_stats.neutral_peer_export,
                up_down_hill_stats.neutral_other_export,
                up_down_hill_stats.bad_up_import,
                up_down_hill_stats.bad_down_import,
                up_down_hill_stats.bad_peer_import,
                up_down_hill_stats.bad_other_import,
                up_down_hill_stats.bad_up_export,
                up_down_hill_stats.bad_down_export,
                up_down_hill_stats.bad_peer_export,
                up_down_hill_stats.bad_other_export,
            ],
        ),
    ])?;
    CsvWriter::new(File::create("up_down_hill_stats.csv")?).finish(&mut up_down_hill_df)?;

    Ok(())
}

/// Generate statistics for each AS.
/// Copy this after running code from [`parse_bgp_lines`],
/// except maybe the `db` line.
fn gen_as_stats(query: QueryDump, mut bgp_lines: Vec<Line>) -> Result<()> {
    let start = Instant::now();
    let map: DashMap<u64, AsStats> = DashMap::new();
    bgp_lines.par_iter_mut().for_each(|l| {
        l.compare.as_stats(&query, &map);
    });
    let size = map.len();
    println!(
        "Generated stats for {size} AS in {}ms.",
        start.elapsed().as_millis()
    );
    let (ans, ioks, eoks, isps, esps, iers, eers): (
        Vec<u64>,
        Vec<u32>,
        Vec<u32>,
        Vec<u32>,
        Vec<u32>,
        Vec<u32>,
        Vec<u32>,
    ) = multiunzip(map.into_iter().map(
        |(
            an,
            AsStats {
                import_ok,
                export_ok,
                import_skip,
                export_skip,
                import_err,
                export_err,
            },
        )| {
            (
                an,
                import_ok,
                export_ok,
                import_skip,
                export_skip,
                import_err,
                export_err,
            )
        },
    ));

    let mut df = DataFrame::new(vec![
        Series::new("aut_num", ans),
        Series::new("import_ok", ioks),
        Series::new("export_ok", eoks),
        Series::new("import_skip", isps),
        Series::new("export_skip", esps),
        Series::new("import_err", iers),
        Series::new("export_err", eers),
    ])?;
    println!("{df}");
    println!("{}", df.describe(None)?);

    CsvWriter::new(File::create("as_stats.csv")?).finish(&mut df)?;

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
