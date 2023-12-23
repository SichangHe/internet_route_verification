use std::{
    fs::{read_dir, File},
    io::{BufWriter, Write},
    path::Path,
    sync::mpsc::channel,
    thread::spawn,
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
        stats::{as_, as_pair, csv_header, route, AsPairStats, RouteStats},
        QueryIr, Verbosity,
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

    let mut route_stats_file =
        BufWriter::new(File::create(format!("{collector}--route_stats.csv"))?);
    route_stats_file.write_all(csv_header().trim_end_matches(',').as_bytes())?;
    route_stats_file.write_all(b"\n")?;

    let start = Instant::now();
    let as_stats_map: DashMap<u32, RouteStats<u64>> = DashMap::new();
    let as_pair_map: DashMap<(u32, u32), AsPairStats> = DashMap::new();
    let n_route_stats = bgp_lines.len();

    let (route_stats_sender, route_stats_receiver) = channel::<RouteStats<_>>();
    let route_stats_writer = spawn(move || -> Result<_> {
        while let Ok(stats) = route_stats_receiver.recv() {
            route_stats_file.write_all(&stats.as_csv_bytes())?;
            route_stats_file.write_all(b"\n")?;
        }
        route_stats_file.flush()?;

        Ok(())
    });

    bgp_lines.into_par_iter().for_each(|line| {
        let compare = line.compare.verbosity(Verbosity {
            record_community: true,
            ..Verbosity::minimum_all()
        });
        let reports = compare.check_with_relationship(query, db);

        let mut stats = RouteStats::default();
        for report in &reports {
            as_::one(&as_stats_map, report);
            as_pair::one(db, &as_pair_map, report);
            route::one(&mut stats, report);
        }

        route_stats_sender.send(stats).unwrap();
    });
    drop(route_stats_sender); // Close channel.

    println!(
        "Generated stats for {} ASes, {} AS pairs, {} routes in {}.",
        as_stats_map.len(),
        as_pair_map.len(),
        n_route_stats,
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

    route_stats_writer.join().unwrap()?;
    debug!("Wrote route stats for `{collector}`.");

    Ok(())
}
