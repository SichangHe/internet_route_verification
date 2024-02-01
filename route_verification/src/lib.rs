use anyhow::{bail, Result};
use log::debug;

pub use {as_rel, bgp, common_regex, io, ir, irr, lex, parse};

mod fs;

pub fn parse_one(args: Vec<String>) -> Result<()> {
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

    let (parsed, counts) = fs::parse_all(input_dir)?;
    println!("Summary\n\tParsed {parsed}.\n\t{counts}.");

    debug!("Starting to write the parsed IR.");
    parsed.pal_write(output_dir)?;
    debug!("Wrote the parsed IR.");

    Ok(())
}

pub fn parse_priority(args: Vec<String>) -> Result<()> {
    if args.len() < 4 {
        bail!("Specify directories to read from in descending order of priorities, and a directory to write to!");
    }

    let input_dirs = &args[2..args.len() - 1];
    debug!("Will read from {:?}.", input_dirs);
    let output_dir = &args.last().expect("`args.len() >= 5`");
    debug!("Will dump to {output_dir}.");

    fs::parse_priority(input_dirs, output_dir)
}

pub fn parse_ordered(args: Vec<String>) -> Result<()> {
    if args.len() < 4 {
        bail!("Specify DB files to read from in descending order of priorities, and a directory to write to!");
    }

    let input_dbs = &args[2..args.len() - 1];
    debug!("Will read from {:?}.", input_dbs);
    let output_dir = &args.last().expect("`args.len() >= 5`");
    debug!("Will dump to {output_dir}.");

    fs::parse_ordered(input_dbs, output_dir)
}

pub fn report(args: Vec<String>) -> Result<()> {
    if args.len() < 4 {
        bail!("Specify a directory to read parsed IR from and a MRT file to read from!");
    }

    let parsed_dir = &args[2];
    debug!("Will read parsed IR from {parsed_dir}.");

    let mrt_dir = &args[3];
    debug!("Will read MRT file from {mrt_dir}.");

    fs::report(parsed_dir, mrt_dir)
}

#[cfg(test)]
mod evcxr_examples;
