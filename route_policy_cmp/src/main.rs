use anyhow::{bail, Result};

use encoding_rs::Encoding;
use encoding_rs_io::DecodeReaderBytesBuilder;
use log::debug;
use route_policy_cmp::{irr::read_db, parse::lex::parse_lexed};
use std::{
    env::args,
    fs::File,
    io::{stdout, BufReader},
};

fn main() -> Result<()> {
    // TODO: Make a shell.
    env_logger::init();
    let args: Vec<_> = args().collect();
    if args.len() < 2 {
        bail!("Specify a file to read!");
    }

    let filename = &args[1];
    debug!("Will read from {filename}.");
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
    serde_json::to_writer(stdout(), &parsed)?;
    debug!("Wrote the parsed dump.");

    Ok(())
}
