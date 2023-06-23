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

    let mut bgp_lines: Vec<Line> = parse_mrt("data/mrts/rib.20230619.2200.bz2")?;

    bgp_lines.first();

    bgp_lines[0].compare.check(&parsed);

    // TODO: Below line maximizes out all CPUs and causes memory outage.
    bgp_lines
        .par_iter_mut()
        .for_each(|line| line.report = Some(line.compare.check(&parsed)));

    for (index, line) in bgp_lines[..].iter_mut().enumerate() {
        let report = line.compare.check(&parsed);
        if report.is_empty() {
            line.report = Some(report);
        } else {
            line.report = Some(report);
            println!("{index}: {line:#?}");
            break;
        }
    }

    bgp_lines[1].compare.verbosity = Verbosity::Detailed;
    let report = bgp_lines[1].compare.check(&parsed);
    let items: Option<Vec<ReportItem>> = if let Report::Bad(items) = &report[2] {
        Some(items.clone())
    } else {
        None
    };
    let items: Vec<ReportItem> = items.unwrap();

    println!(
        "{:#?}",
        &parsed.aut_nums.get(&3257).unwrap().imports.any.any[401..500]
    );

    // ---

    for (index, line) in bgp_lines[1000..].iter_mut().enumerate() {
        let report = line.compare.check(&parsed);
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
