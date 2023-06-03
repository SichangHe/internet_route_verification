use anyhow::Result;
use log::debug;
use route_policy_cmp::{bgp::cmp::compare_line_w_dump, lex::dump::Dump, parse::lex::parse_lexed};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    env_logger::init();
    // Test lex dumped.
    debug!("Loading lexed dump.");
    let file = File::open("../dump.json")?;
    let lexed = Dump::from_reader(file)?;
    debug!("Loaded lexed dump.");

    let parsed = parse_lexed(lexed);
    debug!("Parsed lexed dump.");
    for line in BufReader::new(File::open("../data/bgp_routes_eg.txt")?).lines() {
        let reports = compare_line_w_dump(&line?, &parsed)?;
        println!("{line}\n{reports:#?}\n");
    }

    Ok(())
}
