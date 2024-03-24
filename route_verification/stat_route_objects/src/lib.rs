use std::{
    fmt::Display,
    fs::{read_dir, File},
    io::BufRead,
};

use anyhow::{bail, Result};
use hashbrown::HashMap;
use log::{debug, error, warn};
use rayon::prelude::*;

use route_verification::{
    fs::open_file_w_correct_encoding,
    irr::split_commas,
    lex::{expressions, io_wrapper_lines, lines_continued, rpsl_objects, RpslExpr},
};
use serde::{Deserialize, Serialize};

pub fn scan_dirs(input_dirs: &[String]) -> Result<()> {
    if input_dirs.is_empty() {
        bail!("No input directories specified.");
    }

    debug!("Starting to scan `{input_dirs:?}`.");
    let all_scanned_routes: Vec<_> = input_dirs
        .par_iter()
        .rev()
        .filter_map(|dir| match scan_dir(dir) {
            Ok(routes) => Some(routes),
            Err(why) => {
                error!("Error scanning {dir}: {why:?}");
                None
            }
        })
        .flatten()
        .collect();
    debug!("Scanned `{input_dirs:?}`.");

    let mut aggregated_routes: HashMap<String, Vec<_>> =
        HashMap::with_capacity(all_scanned_routes.len());
    for route in all_scanned_routes {
        aggregated_routes
            .entry_ref(&route.name)
            .or_default()
            .push(route);
    }

    let total_n_route = aggregated_routes.len();
    debug!("Aggregated {total_n_route} routes.");

    let routes_defined_multiple_times: HashMap<_, _> = aggregated_routes
        .iter()
        .filter(|(_, routes)| routes.len() > 1)
        .collect();
    debug!(
        "{} routes defined multiple times.",
        routes_defined_multiple_times.len()
    );

    let route_defined_by_different_mntners: HashMap<_, _> = routes_defined_multiple_times
        .iter()
        .filter(|(_, routes)| {
            let first_route = &routes[0];
            routes[1..]
                .iter()
                .any(|route| route.mnt_by != first_route.mnt_by)
        })
        .collect();
    debug!(
        "{} routes defined by multiple maintainers.",
        route_defined_by_different_mntners.len()
    );

    warn!("Dumping routes defined multiple times.");
    let mut file = File::create("route_objects_defined_multiple_times.json")?;
    serde_json::to_writer(&mut file, &routes_defined_multiple_times)?;

    // The other two maps can also be dumped.
    Ok(())
}

pub fn scan_dir(input_dir: &str) -> Result<Vec<Route>> {
    debug!("Starining to scan {input_dir}.");
    let routes_in_dir = read_dir(input_dir)?
        .par_bridge()
        .map(|entry| {
            let path = entry?.path();
            let reader = open_file_w_correct_encoding(&path)?;
            let tag = path.to_string_lossy();
            scan_db(tag, reader)
        })
        .filter_map(|maybe_routes| match maybe_routes {
            Ok(routes) => Some(routes),
            Err(why) => {
                error!("Error scanning {input_dir}: {why:?}");
                None
            }
        })
        .flatten()
        .collect();

    debug!("Scanned {input_dir}.");
    Ok(routes_in_dir)
}

#[derive(Deserialize, Serialize)]
pub struct Route {
    pub name: String,
    pub origin: Option<String>,
    pub mnt_by: Vec<String>,
    pub source: Option<String>,
}

pub fn scan_db(tag: impl Display, db: impl BufRead) -> Result<Vec<Route>> {
    debug!("Starting to scan {tag}.");
    let mut routes = Vec::new();

    for obj in rpsl_objects(io_wrapper_lines(db)) {
        if !matches!(obj.class.as_str(), "route" | "route6") {
            continue;
        }

        let mut origin = None;
        let mut source = None;
        let mut mnt_by = Vec::new();
        for RpslExpr { key, expr } in expressions(lines_continued(obj.body.lines())) {
            match key.as_str() {
                "origin" => origin = Some(expr),
                "mnt-by" => mnt_by.extend(split_commas(&expr).map(str::to_string)),
                "source" => source = Some(expr),
                _ => {}
            }
        }
        mnt_by.shrink_to_fit();

        routes.push(Route {
            name: obj.name,
            origin,
            mnt_by,
            source,
        })
    }
    debug!("Scanned {tag}.");

    Ok(routes)
}
