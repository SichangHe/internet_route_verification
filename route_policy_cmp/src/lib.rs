use anyhow::{bail, Result};
use log::debug;
use rayon::prelude::*;

use crate::{bgp::*, parse::dump::Dump};

pub mod bgp;
pub mod cmd;
pub mod fs;
pub mod irr;
pub mod lex;
pub mod parse;
pub mod serde;

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

pub fn report(args: Vec<String>) -> Result<()> {
    if args.len() < 4 {
        bail!("Specify a directory to read parsed dump from and a MRT file to read from!");
    }

    let parsed_dir = &args[2];
    debug!("Will read parsed dump from {parsed_dir}.");

    let mrt_dir = &args[3];
    debug!("Will read MRT file from {mrt_dir}.");

    let parsed = Dump::pal_read(parsed_dir)?;
    parsed.log_count();

    let mut bgp_lines: Vec<Line> = parse_mrt(mrt_dir)?;
    const SIZE: usize = 0x100;
    bgp_lines[..SIZE]
        .par_iter_mut()
        .for_each(|line| line.report = Some(line.compare.check(&parsed)));

    let n_error: usize = bgp_lines[..SIZE]
        .par_iter()
        .map(|line| {
            if line.report.as_ref().unwrap().is_empty() {
                0
            } else {
                1
            }
        })
        .sum();
    println!("{n_error} errors reported in {SIZE} routes.");

    Ok(())
}

#[cfg(test)]
mod test;
