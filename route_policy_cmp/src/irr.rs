pub mod worker;

use std::{
    collections::BTreeMap,
    io::{BufReader, Read},
    process::ChildStdout,
    sync::mpsc::Sender,
};

use crate::{
    lex::{
        dump::Dump,
        lines::{
            expressions, io_wrapper_lines, lines_continued, rpsl_objects, RPSLObject, RpslExpr,
        },
        rpsl_object::AsOrRouteSet,
    },
    parse::{
        dump::{self, merge_dumps},
        lex::parse_lexed,
    },
};

use anyhow::Result;
use lazy_regex::regex;
use log::{debug, error, warn};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use self::worker::{spawn_aut_num_worker, spawn_filter_set_worker, spawn_peering_set_worker};

pub fn gather_members(body: &str) -> Vec<String> {
    let mut members = Vec::new();
    for RpslExpr { key, expr } in expressions(lines_continued(body.lines())) {
        if key == "members" || key == "mp-members" {
            members.extend(regex!(r",\s*").split(&expr).filter_map(|s| {
                let r = s.trim();
                (!r.is_empty()).then(|| r.into())
            }));
        }
    }
    members
}

pub fn read_line_wait(reader: &mut BufReader<ChildStdout>) -> Result<String> {
    let mut line = Vec::new();
    loop {
        let mut buf = [0];
        reader.read_exact(&mut buf)?;
        match buf[0] {
            b'\n' => break,
            b => line.push(b),
        }
    }
    Ok(String::from_utf8(line)?)
}

pub fn parse_object(
    obj: RPSLObject,
    as_sets: &mut Vec<AsOrRouteSet>,
    route_sets: &mut Vec<AsOrRouteSet>,
    send_aut_num: &mut Sender<RPSLObject>,
    send_peering_set: &mut Sender<RPSLObject>,
    send_filter_set: &mut Sender<RPSLObject>,
    as_routes: &mut BTreeMap<String, Vec<String>>,
) -> Result<()> {
    match obj.class.as_str() {
        "aut-num" => send_aut_num.send(obj)?,
        "as-set" => parse_as_set(obj, as_sets),
        "route" | "route6" => parse_route(obj, as_routes),
        "route-set" => parse_route_set(obj, route_sets),
        "filter-set" => send_filter_set.send(obj)?,
        "peering-set" => send_peering_set.send(obj)?,
        _ => (),
    }
    Ok(())
}

fn parse_as_set(obj: RPSLObject, as_sets: &mut Vec<AsOrRouteSet>) {
    let members = gather_members(&obj.body);
    as_sets.push(AsOrRouteSet::new(obj.name, obj.body, members));
    match as_sets.len() {
        l if l % 0xFF == 0 => debug!("Parsed {l} as_sets."),
        _ => (),
    }
}

fn parse_route(obj: RPSLObject, as_routes: &mut BTreeMap<String, Vec<String>>) {
    for RpslExpr {
        key,
        expr, /*AS*/
    } in expressions(lines_continued(obj.body.lines()))
    {
        if key == "origin" {
            as_routes
                .entry(expr.to_uppercase())
                .or_default()
                .push(obj.name /*The route*/);
            return;
        }
    }
    error!("Route object {} does not have an `origin` field.", obj.name);
}

fn parse_route_set(obj: RPSLObject, route_sets: &mut Vec<AsOrRouteSet>) {
    let members = gather_members(&obj.body);
    route_sets.push(AsOrRouteSet::new(obj.name, obj.body, members));
    match route_sets.len() {
        l if l % 0xFF == 0 => debug!("Parsed {l} route_sets."),
        _ => (),
    }
}

const ONE_MEBIBYTE: usize = 1024 * 1024;

pub fn read_db<R>(db: BufReader<R>) -> Result<Dump>
where
    R: Read,
{
    let (mut as_sets, mut route_sets, mut as_routes) = (Vec::new(), Vec::new(), BTreeMap::new());
    let (mut send_aut_num, aut_num_worker) = spawn_aut_num_worker()?;
    let (mut send_peering_set, peering_set_worker) = spawn_peering_set_worker()?;
    let (mut send_filter_set, filter_set_worker) = spawn_filter_set_worker()?;

    for obj in rpsl_objects(io_wrapper_lines(db)) {
        if obj.body.len() > ONE_MEBIBYTE {
            // <https://github.com/SichangHe/parse_rpsl_policy/issues/6#issuecomment-1566121009>
            warn!(
                "Skipping {} object `{}` with body larger than 1MiB.",
                obj.class, obj.name
            );
            continue;
        }

        parse_object(
            obj,
            &mut as_sets,
            &mut route_sets,
            &mut send_aut_num,
            &mut send_peering_set,
            &mut send_filter_set,
            &mut as_routes,
        )?;
    }

    drop((send_aut_num, send_peering_set, send_filter_set));
    let aut_nums = aut_num_worker.join().unwrap()?;
    let peering_sets = peering_set_worker.join().unwrap()?;
    let filter_sets = filter_set_worker.join().unwrap()?;

    Ok(Dump {
        aut_nums,
        as_sets,
        route_sets,
        peering_sets,
        filter_sets,
        as_routes,
    })
}

/// When some DBs have the same keys, any value could be used.
pub fn parse_dbs<I, R>(dbs: I) -> Result<dump::Dump>
where
    I: IntoParallelIterator<Item = BufReader<R>>,
    R: Read,
{
    let dumps = dbs
        .into_par_iter()
        .map(|db| read_db(db).map(parse_lexed))
        .collect::<Result<_>>()?;
    Ok(merge_dumps(dumps))
}
