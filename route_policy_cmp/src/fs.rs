use anyhow::Result;
use chardetng::EncodingDetector;
use encoding_rs::Encoding;
use encoding_rs_io::DecodeReaderBytesBuilder;
use log::debug;
use rayon::prelude::*;
use std::{fs::*, io::*, path::Path};

use crate::{
    irr::*,
    parse::{dump::Dump, lex::parse_lexed},
};

pub fn parse(filename: &str, output_dir: &str) -> Result<()> {
    let encoding = detect_file_encoding(filename)?;
    let decoder = DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding))
        .build(File::open(filename)?);
    let reader = BufReader::new(decoder);
    let dump = read_db(reader)?;
    dump.log_count();

    let parsed = parse_lexed(dump);
    debug!("Starting to write the parsed dump.");
    parsed.pal_write(output_dir)?;
    debug!("Wrote the parsed dump.");

    Ok(())
}

pub fn read(input_dir: &str) -> Result<()> {
    let dump = Dump::pal_read(input_dir)?;
    dump.log_count();
    dump.split_n_cpus()?;
    Ok(())
}

pub fn parse_all(input_dir: &str, output_dir: &str) -> Result<()> {
    let readers = read_dir(input_dir)?
        .par_bridge()
        .map(|entry| {
            let path = entry?.path();
            let encoding = detect_file_encoding(&path)?;
            let decoder = DecodeReaderBytesBuilder::new()
                .encoding(Some(encoding))
                .build(File::open(path)?);
            let reader = BufReader::new(decoder);
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

pub fn detect_file_encoding<P>(path: P) -> Result<&'static Encoding>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let mut file = File::open(path).expect("Failed to open file");
    let mut buffer = [0; 1024];
    let mut detector = EncodingDetector::new();
    let mut total_read = 0;

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            let encoding = detector.guess(None, true);
            debug!(
                "Guessing that {} is encoded in {} after reading all {total_read} bytes.",
                path.to_str().unwrap_or_default(),
                encoding.name()
            );
            return Ok(encoding);
        }
        total_read += bytes_read;
        if !detector.feed(&buffer[..bytes_read], false) {
            continue;
        }
        if let (encoding, true) = detector.guess_assess(None, true) {
            debug!(
                "Detected that {} is encoded in {} after reading {total_read} bytes.",
                path.to_str().unwrap_or_default(),
                encoding.name()
            );
            return Ok(encoding);
        }
    }
}
