//! Notebook usage example snippets.
#![allow(dead_code)]
#![allow(clippy::no_effect)]
#![allow(unused_must_use)]
#![allow(unused_variables)]

use super::*;
use crate as route_policy_cmp;

use rayon::prelude::*;
use route_policy_cmp::{bgp::*, parse::dump::Dump};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    time::Instant,
};

fn read_parsed_rpsl() -> Result<()> {
    let parsed = Dump::pal_read("parsed_all")?;
    let query = QueryDump::from_dump(parsed);

    let bgp_file: Vec<String> = BufReader::new(File::open("data/bgp_routes_eg.txt")?)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    // Remove `;` in notebook.
    Compare::with_line_dump(&bgp_file[2])?.check(&query);

    Verbosity::Brief > Verbosity::ErrOnly;

    Ok(())
}

fn parse_bgp_lines() -> Result<()> {
    let parsed = Dump::pal_read("parsed_all")?;
    let query: QueryDump = QueryDump::from_dump(parsed);

    query.aut_nums.iter().next();

    let mut bgp_lines: Vec<Line> = parse_mrt("data/mrts/rib.20230619.2200.bz2")?;

    // ---
    // Generate all the reports:
    let start = Instant::now();
    bgp_lines.par_iter_mut().for_each(|line| line.check(&query));
    println!("Used {}ms", start.elapsed().as_millis());

    // Statistics on number of bad/neutral/good routes.
    let bad_neutral_good = bgp_lines
        .par_iter_mut()
        .map(|l| {
            if let Some(report) = &l.report {
                if !report.is_empty() {
                    return (1, 0, 0);
                }
            }
            l.compare.verbosity = Verbosity::ShowSkips;
            let report = l.compare.check(&query);
            match report.iter().any(|r| matches!(r, Report::Neutral(_))) {
                true => (0, 1, 0),
                false => (0, 0, 1),
            }
        })
        .reduce(|| (0, 0, 0), |(x, y, z), (a, b, c)| (x + a, y + b, z + c));

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

    bgp_lines[1].compare.verbosity = Verbosity::Detailed;
    let report = bgp_lines[1].compare.check(&query);
    let items: Option<Vec<ReportItem>> = if let Report::Bad(items) = &report[2] {
        Some(items.clone())
    } else {
        None
    };
    let items: Vec<ReportItem> = items.unwrap();

    println!(
        "{:#?}",
        &query.aut_nums.get(&3257).unwrap().imports.any.any[401..500]
    );

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
