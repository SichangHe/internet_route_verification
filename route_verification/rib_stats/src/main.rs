use std::{
    fs::{create_dir, read_dir, File},
    io::{BufRead, BufWriter, Write},
    mem,
    path::Path,
    sync::mpsc::{channel, sync_channel},
    thread::spawn,
    time::Instant,
};

use anyhow::Result;
use dashmap::DashMap;
use flate2::{write::GzEncoder, Compression};
use human_duration::human_duration;
use log::{debug, error, info};
use rayon::prelude::*;
use route_verification::{
    as_rel::{AsRelDb, Relationship},
    bgp::{
        stats::{as_, as_pair, csv_header, route, AsPairStats, RouteStats},
        wrapper::read_mrt,
        Line, QueryIr, Verbosity,
    },
    ir::Ir,
};

fn main() {
    env_logger::init_from_env(
        // Set default log level to "debug".
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "debug"),
    );
    info!("Starting...");

    let db = AsRelDb::load_bz("../../data/20230701.as-rel.bz2").unwrap();
    let parsed = Ir::pal_read("../../parsed_all").unwrap();
    let query = QueryIr::from_ir_and_as_relationship(parsed, &db);
    debug!("Loaded AS Relationship DB and IR for query");

    let rib_files = read_dir("../../data/ribs")
        .unwrap()
        .map(|maybe_entry| maybe_entry.unwrap().path())
        .filter(|path| {
            path.is_file() && {
                let extension = path.extension().unwrap();
                extension == "gz" || extension == "bz2"
            }
        })
        .collect::<Vec<_>>();

    _ = create_dir("all5");

    let mut failed = vec![];
    for rib_file in &rib_files {
        match process_rib_file(&query, &db, rib_file) {
            Ok(_) => (),
            Err(why) => {
                error!("Failed to process {}: {why:?}", rib_file.display());
                failed.push(rib_file);
            }
        }
    }

    if failed.is_empty() {
        println!(
            "Successfully generated stats for {} RIB files.",
            rib_files.len()
        );
    } else {
        println!(
            "Summary:
\tSuccessfully generated stats for {} RIB files.
\tFailed to generate stats for {} RIB files: {failed:?}.",
            rib_files.len() - failed.len(),
            failed.len()
        );
    }
}

fn process_rib_file(query: &QueryIr, db: &AsRelDb, rib_file: &Path) -> Result<()> {
    let rib_file_name = rib_file
        .file_name()
        .expect("RIB file should have a name.")
        .to_string_lossy();
    let collector = rib_file_name
        .split("--")
        .next()
        .expect("First split always succeeds.");

    let route_stats_filename = format!("all5/{collector}--route_stats4.csv.gz");
    let route_first_hop_stats_filename = format!("all5/{collector}--route_first_hop_stats4.csv.gz");
    let as_stats_filename = format!("all5/{collector}--as_stats4.csv.gz");
    let as_pair_stats_filename = format!("all5/{collector}--as_pair_stats4.csv.gz");
    if [
        &route_stats_filename,
        &route_first_hop_stats_filename,
        &as_stats_filename,
        &as_pair_stats_filename,
    ]
    .into_iter()
    .all(|name| Path::new(name).exists())
    {
        debug!("Skipping processed RIB file `{rib_file_name}` for collector `{collector}`.");
        return Ok(());
    }

    debug!("Starting to process RIB file `{rib_file_name}` for collector `{collector}`.");
    // Bounded channel to apply back pressure to bgpdump Stdout.
    let (line_sender, line_receiver) = sync_channel(32);
    let mut bgpdump_child = read_mrt(rib_file)?;
    let bgpdump_handler = spawn(move || {
        let mut line = String::new();

        while bgpdump_child
            .stdout
            .read_line(&mut line)
            .expect("Error reading `bgpdump` output.")
            > 0
        {
            line_sender
                .send(mem::take(&mut line))
                .expect("`line_receiver` should stay open.");
        }
    });

    let start = Instant::now();
    let as_stats_map: DashMap<u32, RouteStats<u64>> = DashMap::new();
    let as_pair_map: DashMap<(u32, u32), AsPairStats> = DashMap::new();
    let csv_header = csv_header();

    let (route_stats_sender, route_stats_receiver) = channel::<RouteStats<_>>();
    let route_stats_writer = {
        let mut route_stats_file = gzip_file(route_stats_filename)?;
        route_stats_file.write_all(csv_header.trim_end_matches(',').as_bytes())?;
        route_stats_file.write_all(b"\n")?;

        spawn(move || -> Result<_> {
            while let Ok(stats) = route_stats_receiver.recv() {
                route_stats_file.write_all(&stats.as_csv_bytes())?;
                route_stats_file.write_all(b"\n")?;
            }
            route_stats_file.flush()?;

            Ok(())
        })
    };

    let (route_first_hop_stats_sender, route_first_hop_stats_receiver) = channel::<RouteStats<_>>();
    let route_first_hop_stats_writer = {
        let mut route_first_hop_stats_file = gzip_file(route_first_hop_stats_filename)?;
        route_first_hop_stats_file.write_all(csv_header.trim_end_matches(',').as_bytes())?;
        route_first_hop_stats_file.write_all(b"\n")?;

        spawn(move || -> Result<_> {
            while let Ok(stats) = route_first_hop_stats_receiver.recv() {
                route_first_hop_stats_file.write_all(&stats.as_csv_bytes())?;
                route_first_hop_stats_file.write_all(b"\n")?;
            }
            route_first_hop_stats_file.flush()?;

            Ok(())
        })
    };

    let n_route_stats = line_receiver
        .into_iter()
        .par_bridge()
        .map(|line| {
            let line = Line::from_raw(line).expect("`bgpdump` should output valid lines.");
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
            route_stats_sender
                .send(stats)
                .expect("`route_stats_sender` should not have been closed.");

            let mut first_hop_stats = RouteStats::default();
            // Assume that reports for the first hop are the first two.
            for report in reports.iter().take(2) {
                route::one(&mut first_hop_stats, report)
            }
            route_first_hop_stats_sender
                .send(first_hop_stats)
                .expect("`route_first_hop_stats_sender` should not have been closed.");
        })
        .count();
    drop((route_stats_sender, route_first_hop_stats_sender)); // Close channels.

    bgpdump_handler
        .join()
        .expect("`bgpdump_handler` should not panic.");

    println!(
        "Generated stats for {} ASes, {} AS pairs, {n_route_stats} routes for {collector} in {}.",
        as_stats_map.len(),
        as_pair_map.len(),
        human_duration(&start.elapsed())
    );

    route_stats_writer
        .join()
        .expect("Route stats writer thread should not panic.")?;
    route_first_hop_stats_writer
        .join()
        .expect("Route stats writer thread should not panic.")?;
    debug!("Wrote route stats for `{collector}`.");

    {
        let start = Instant::now();
        let mut as_stats_file = gzip_file(as_stats_filename)?;
        as_stats_file.write_all(b"aut_num,")?;
        as_stats_file.write_all(csv_header.trim_end_matches(',').as_bytes())?;
        as_stats_file.write_all(b"\n")?;

        for (an, s) in as_stats_map.into_iter() {
            as_stats_file.write_all(an.to_string().as_bytes())?;
            as_stats_file.write_all(b",")?;
            as_stats_file.write_all(&s.as_csv_bytes())?;
            as_stats_file.write_all(b"\n")?;
        }
        as_stats_file.flush()?;
        debug!(
            "Wrote AS stats for `{collector}` in {}.",
            human_duration(&start.elapsed())
        );
    }

    {
        let start = Instant::now();
        let mut as_pair_stats_file = gzip_file(as_pair_stats_filename)?;
        as_pair_stats_file.write_all(b"from,to,")?;
        as_pair_stats_file.write_all(csv_header.as_bytes())?;
        as_pair_stats_file.write_all(b"relationship\n")?;

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
        debug!(
            "Wrote AS pair stats for `{collector}` in {}.",
            human_duration(&start.elapsed())
        );
    }

    Ok(())
}

fn gzip_file(path: impl AsRef<Path>) -> Result<GzEncoder<BufWriter<File>>> {
    Ok(GzEncoder::new(
        BufWriter::new(File::create(path)?),
        Compression::default(),
    ))
}
