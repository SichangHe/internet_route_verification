use std::{fs::*, io::*, path::Path};

use chardetng::EncodingDetector;
use encoding_rs::Encoding;
use encoding_rs_io::DecodeReaderBytesBuilder;
use lex::Counts;
use rayon::prelude::*;

use super::{
    bgp::*,
    irr::*,
    parse::{parse_lexed, Dump},
    Result, *,
};

pub fn parse(filename: &str, output_dir: &str) -> Result<()> {
    let encoding = detect_file_encoding(filename)?;
    let decoder = DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding))
        .build(File::open(filename)?);
    let reader = BufReader::new(decoder);
    let (dump, l_counts) = read_db(reader)?;
    let (parsed, p_counts) = parse_lexed(dump);
    let counts = l_counts + p_counts;
    println!("Summary\n\tParsed {parsed}.\n\t{counts}.");
    debug!("Starting to write the parsed dump.");
    parsed.pal_write(output_dir)?;
    debug!("Wrote the parsed dump.");

    Ok(())
}

pub fn read(input_dir: &str) -> Result<()> {
    let dump = Dump::pal_read(input_dir)?;
    debug!("read: Parsed {dump}.");
    dump.split_n_cpus()?;
    Ok(())
}

pub fn parse_all(input_dir: &str) -> Result<(Dump, Counts)> {
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
    parse_dbs(readers)
}
pub fn parse_priority(priority_dir: &str, backup_dir: &str, output_dir: &str) -> Result<()> {
    let (priority, p_counts) = parse_all(priority_dir)?;
    let (backup, b_counts) = parse_all(backup_dir)?;
    let parsed = backup.merge(priority);
    let counts = p_counts + b_counts;
    println!("Summary\n\tParsed {parsed}.\n\t{counts}.");

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

pub fn report(parsed_dir: &str, mrt_dir: &str) -> Result<()> {
    let parsed = Dump::pal_read(parsed_dir)?;
    debug!("report: Parsed {parsed}.");

    let query = QueryDump::from_dump(parsed);
    debug!("Converted Dump to QueryDump");

    let mut bgp_lines = parse_mrt(mrt_dir)?;
    debug!("Read {} lines from {mrt_dir}", bgp_lines.len());

    const SIZE: usize = 0x1000;
    bgp_lines[..SIZE].iter_mut().for_each(|line| {
        line.compare.verbosity = Verbosity::minimum_all();
        line.check(&query);
    });
    debug!("Generated {SIZE} reports");

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
