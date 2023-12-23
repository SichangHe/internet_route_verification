use std::{
    fs::{read_dir, File},
    io::{BufWriter, Write},
    path::Path,
    time::Instant,
};

use anyhow::Result;
use dashmap::DashMap;
use human_duration::human_duration;
use log::{debug, info};
use rayon::prelude::*;
use route_verification::{
    as_rel::{AsRelDb, Relationship},
    bgp::{
        parse_mrt,
        stats::{csv_header, AsPairStats, RouteStats},
        QueryIr,
    },
    ir::Ir,
};

fn main() {
    env_logger::init_from_env(
        // Set default log level to "debug".
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "debug"),
    );
    info!("Starting...");

    let db = AsRelDb::load_bz("data/20230701.as-rel.bz2").unwrap();
    let parsed = Ir::pal_read("parsed_all").unwrap();
    let query = QueryIr::from_ir_and_as_relationship(parsed, &db);
    debug!("Loaded AS Relationship DB and IR for query");

    let rib_files = read_dir("data/ribs")
        .unwrap()
        .map(|maybe_entry| maybe_entry.unwrap().path())
        .filter(|path| path.is_file() && (path.ends_with(".gz") || path.ends_with(".bz2")))
        .collect::<Vec<_>>();

    for rib_file in rib_files {
        process_rib_file(&query, &db, &rib_file).unwrap();
    }
}

fn process_rib_file(query: &QueryIr, db: &AsRelDb, rib_file: &Path) -> Result<()> {
    let rib_file_name = rib_file.to_string_lossy();
    let collector = rib_file_name.split("--").next().unwrap();
    debug!("Starting to process rib file `{rib_file_name}` for collector `{collector}`.");

    let start = Instant::now();
    let mut bgp_lines = parse_mrt(rib_file).unwrap();
    debug!(
        "Parsed {rib_file_name} in {}.",
        human_duration(&start.elapsed())
    );

    let mut as_stats_file = BufWriter::new(File::create(format!("{collector}--as_stats.csv"))?);
    as_stats_file.write_all(b"aut_num,")?;
    as_stats_file.write_all(csv_header().trim_end_matches(',').as_bytes())?;
    as_stats_file.write_all(b"\n")?;

    let mut as_pair_stats_file =
        BufWriter::new(File::create(format!("{collector}--as_pair_stats.csv"))?);
    as_pair_stats_file.write_all(b"from,to,")?;
    as_pair_stats_file.write_all(csv_header().as_bytes())?;
    as_pair_stats_file.write_all(b"relationship\n")?;

    let mut file = BufWriter::new(File::create(format!("{collector}--route_stats.csv"))?);

    let start = Instant::now();
    // TODO: Inline stats generation.
    let as_stats_map: DashMap<u32, RouteStats<u64>> = DashMap::new();
    let as_pair_map: DashMap<(u32, u32), AsPairStats> = DashMap::new();

    bgp_lines.par_iter_mut().for_each(|l| {
        l.compare.as_stats(query, db, &as_stats_map);
    });
    bgp_lines.par_iter_mut().for_each(|l| {
        l.compare.as_pair_stats(query, db, &as_pair_map);
    });
    let stats: Vec<RouteStats<u16>> = bgp_lines
        .par_iter_mut()
        .map(|line| line.compare.route_stats(&query, &db))
        .collect();

    file.write_all(csv_header().trim_end_matches(',').as_bytes())?;
    file.write_all(b"\n")?;

    println!(
        "Generated stats for {} ASes, {} AS pairs, {} routes in {}.",
        as_stats_map.len(),
        as_pair_map.len(),
        stats.len(),
        human_duration(&start.elapsed())
    );

    for (an, s) in as_stats_map.into_iter() {
        as_stats_file.write_all(an.to_string().as_bytes())?;
        as_stats_file.write_all(b",")?;
        as_stats_file.write_all(&s.as_csv_bytes())?;
        as_stats_file.write_all(b"\n")?;
    }
    as_stats_file.flush()?;
    debug!("Wrote AS stats for `{collector}`.");

    for (
        (from, to),
        AsPairStats {
            route_stats,
            relationship,
        },
    ) in as_pair_map.into_iter()
    {
        as_pair_stats_file.write_all(format!("{from},{to},").as_bytes())?;
        as_pair_stats_file.write_all(&route_stats.as_csv_bytes())?;
        as_pair_stats_file.write_all(b",")?;
        as_pair_stats_file.write_all(match relationship {
            Some(Relationship::P2C) => b"down",
            Some(Relationship::P2P) => b"peer",
            Some(Relationship::C2P) => b"up",
            None => b"other",
        })?;
        as_pair_stats_file.write_all(b"\n")?;
    }
    as_pair_stats_file.flush()?;
    debug!("Wrote AS pair stats for `{collector}`.");

    for s in stats {
        file.write_all(&s.as_csv_bytes())?;
        file.write_all(b"\n")?;
    }
    file.flush()?;
    debug!("Wrote route stats for `{collector}`.");

    Ok(())
}
