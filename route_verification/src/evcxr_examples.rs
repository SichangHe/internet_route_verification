//! Evcxr usage example snippets.
#![allow(dead_code)]
#![allow(clippy::no_effect)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(clippy::type_complexity)]

use super::*;
use crate as route_verification;

/* Copy from the next line until the end of `use`.
:opt 3
:dep dashmap = "5.5.0"
:dep route_verification = { path = "route_verification" }
:dep rayon
:dep polars = { version = "0.31", features = ["describe"] }
:dep itertools = "0.11"
// */
use dashmap::DashMap;
use itertools::multiunzip;
use polars::prelude::*;
use rayon::prelude::*;
use route_verification::{bgp::*, parse::*};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
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

    // ---
    // Generate statistics for each AS:
    let start = Instant::now();
    let map: DashMap<usize, AsStats> = DashMap::new();
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
                an as u64,
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

    // ---
    // Generate all the reports:
    let start = Instant::now();
    bgp_lines.par_iter_mut().for_each(|line| line.check(&query));
    println!("Used {}ms", start.elapsed().as_millis());

    // ---
    // Benchmark for `match_ips`:
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

    // ---
    // Older stuff.

    bgp_lines.first();

    bgp_lines[0].compare.check(&query);

    // TODO: Below line maximizes out all CPUs and causes memory outage.
    bgp_lines
        .par_iter_mut()
        .for_each(|line| line.report = Some(line.compare.check(&query)));

    for (index, line) in bgp_lines[..].iter_mut().enumerate() {
        let report = line.compare.check(&query);
        if report.is_empty() {
            line.report = Some(report);
        } else {
            line.report = Some(report);
            println!("{index}: {line:#?}");
            break;
        }
    }

    // ---

    for (index, line) in bgp_lines[1000..].iter_mut().enumerate() {
        let report = line.compare.check(&query);
        if report.is_empty() {
            line.report = Some(report);
        } else {
            line.report = Some(report);
            println!("{}: {line:#?}", index + 1000);
            break;
        }
    }

    Ok(())
}
