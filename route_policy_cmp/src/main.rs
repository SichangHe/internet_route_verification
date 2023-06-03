use anyhow::Result;
use route_policy_cmp::{
    bgp::{cmp::compare_line_w_dump, map::parse_table_dump, report::Report},
    parse::lex::Dump,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
env_logger::init();
let file = File::open("parsed.json")?;
let parsed = Dump::from_reader(file)?;

    let bgp_file: Vec<_> = BufReader::new(File::open("../data/bgp_routes_eg.txt")?)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let report7 = compare_line_w_dump(&bgp_file[7], &parsed)?;
    println!("{report7:#?}");

    Ok(())
}
