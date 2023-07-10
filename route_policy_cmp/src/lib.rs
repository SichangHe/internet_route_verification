use std::{cell::RefCell, collections::BTreeMap, mem};

use anyhow::{bail, Context, Error, Result};
use ipnet::IpNet;
use log::{debug, error, warn};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

pub mod bgp;
pub mod cmd;
pub mod fs;
pub mod irr;
pub mod lex;
pub mod parse;
pub mod serialize;

thread_local! {
    static RNG: RefCell<ThreadRng> = RefCell::new(thread_rng());
}

pub fn parse(args: Vec<String>) -> Result<()> {
    if args.len() < 4 {
        bail!("Specify a file to read and a directory to write to!");
    }

    let filename = &args[2];
    debug!("Will read from {filename}.");
    let output_dir = &args[3];
    debug!("Will dump to {output_dir}.");
    fs::parse(filename, output_dir)
}

pub fn read(args: Vec<String>) -> Result<()> {
    if args.len() < 3 {
        bail!("Specify a directory to read!");
    }
    let input_dir = &args[2];
    debug!("Will read from {input_dir}.");
    fs::read(input_dir)
}

pub fn parse_all(args: Vec<String>) -> Result<()> {
    if args.len() < 4 {
        bail!("Specify a directory to read from and a directory to write to!");
    }

    let input_dir = &args[2];
    debug!("Will read from {input_dir}.");
    let output_dir = &args[3];
    debug!("Will dump to {output_dir}.");

    let parsed = fs::parse_all(input_dir)?;
    parsed.log_count();

    debug!("Starting to write the parsed dump.");
    parsed.pal_write(output_dir)?;
    debug!("Wrote the parsed dump.");

    Ok(())
}

pub fn parse_priority(args: Vec<String>) -> Result<()> {
    if args.len() < 5 {
        bail!("Specify a priority directory to read from, a backup directory to read from, and a directory to write to!");
    }

    let priority_dir = &args[2];
    debug!("Will read from {priority_dir} as priority.");
    let backup_dir = &args[3];
    debug!("Will read from {backup_dir} as backup.");
    let output_dir = &args[4];
    debug!("Will dump to {output_dir}.");

    fs::parse_priority(priority_dir, backup_dir, output_dir)
}

pub fn report(args: Vec<String>) -> Result<()> {
    if args.len() < 4 {
        bail!("Specify a directory to read parsed dump from and a MRT file to read from!");
    }

    let parsed_dir = &args[2];
    debug!("Will read parsed dump from {parsed_dir}.");

    let mrt_dir = &args[3];
    debug!("Will read MRT file from {mrt_dir}.");

    fs::report(parsed_dir, mrt_dir)
}

/// Between 0.0 and 1.0.
fn random() -> f64 {
    RNG.with(|rng| rng.borrow_mut().gen())
}

#[cfg(test)]
mod test;
