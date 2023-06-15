use anyhow::{bail, Result};

use encoding_rs::Encoding;
use encoding_rs_io::DecodeReaderBytesBuilder;
use log::debug;
use route_policy_cmp::{
    irr::{parse_dbs, read_db},
    parse::{dump::Dump, lex::parse_lexed},
};
use std::{
    env::args,
    fs::{read_dir, File},
    io::BufReader,
};

fn main() -> Result<()> {
    // TODO: Make a shell.
    env_logger::init();
    let args: Vec<_> = args().collect();
    if args.len() < 2 {
        bail!("Specify a command!");
    }
    match args[1].as_str() {
        "parse" => parse(args),
        "parse_all" => parse_all(args),
        "read" => read(args),
        other => bail!("Unknown command {other}!"),
    }
}

fn parse(args: Vec<String>) -> Result<()> {
    if args.len() < 4 {
        bail!("Specify a file to read and a directory to write to!");
    }

    let filename = &args[2];
    debug!("Will read from {filename}.");
    let output_dir = &args[3];
    debug!("Will dump to {output_dir}.");
    let encoding = Encoding::for_label(b"latin1");
    let reader = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(encoding)
            .build(File::open(filename)?),
    );
    let dump = read_db(reader)?;
    dump.log_count();

    let parsed = parse_lexed(dump);
    debug!("Starting to write the parsed dump.");
    parsed.pal_write(output_dir)?;
    debug!("Wrote the parsed dump.");

    Ok(())
}

fn read(args: Vec<String>) -> Result<()> {
    if args.len() < 3 {
        bail!("Specify a directory to read!");
    }
    let input_dir = &args[2];
    debug!("Will read from {input_dir}.");

    let dump = Dump::pal_read(input_dir)?;
    dump.log_count();
    dump.split_n_cpus()?;
    Ok(())
}

fn parse_all(args: Vec<String>) -> Result<()> {
    if args.len() < 4 {
        bail!("Specify a directory to read from and a directory to write to!");
    }

    let input_dir = &args[2];
    debug!("Will read from {input_dir}.");
    let output_dir = &args[3];
    debug!("Will dump to {output_dir}.");

    let encoding = Encoding::for_label(b"latin1");
    let mut builder = DecodeReaderBytesBuilder::new();
    builder.encoding(encoding);

    let readers = read_dir(input_dir)?
        .map(|path| {
            let reader = BufReader::new(builder.build(File::open(path?.path())?));
            Ok(reader)
        })
        .collect::<Result<Vec<_>>>()?;

    debug!("Starting to read and parse.");
    let parsed = parse_dbs(readers)?;
    parsed.log_count();

    debug!("Starting to write the parsed dump.");
    parsed.pal_write(output_dir)?;
    debug!("Wrote the parsed dump.");

    Ok(())
}
