use anyhow::{bail, Result};

use log::debug;
use route_policy_cmp::irr::read_db;
use std::{
    env::args,
    fs::File,
    io::{stdout, BufReader},
};

fn main() -> Result<()> {
    env_logger::init();
    let args: Vec<_> = args().collect();
    if args.len() < 2 {
        bail!("Specify a file to read!");
    }

    let filename = &args[1];
    debug!("Will read from {filename}.");
    let reader = BufReader::new(File::open(filename)?);
    let dump = read_db(reader);

    debug!("Starting to write the dump.");
    serde_json::to_writer(stdout(), &dump)?;

    Ok(())
}
