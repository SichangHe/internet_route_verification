pub mod worker;

use std::{
    io::{BufReader, Read},
    process::ChildStdout,
    sync::mpsc::Sender,
};

use crate::lex::{
    dump::Dump,
    lines::{expressions, io_wrapper_lines, lines_continued, rpsl_objects, RPSLObject, RpslExpr},
    rpsl_object::AsOrRouteSet,
};

use anyhow::Result;
use log::debug;

use self::worker::{spawn_aut_num_worker, spawn_peering_set_worker};

pub fn gather_members(body: &str) -> Vec<String> {
    let mut members = Vec::new();
    for RpslExpr { key, expr } in expressions(lines_continued(body.lines())) {
        if key == "members" || key == "mp-members" {
            members.extend(expr.split(',').map(|s| s.trim().into()));
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
) -> Result<()> {
    if obj.class == "aut-num" {
        send_aut_num.send(obj)?;
    } else if obj.class == "as-set" {
        let members = gather_members(&obj.body);
        as_sets.push(AsOrRouteSet::new(obj.name, obj.body, members));
        match as_sets.len() {
            l if l % 0xFF == 0 => debug!("Parsed {l} as_sets."),
            _ => (),
        }
    } else if obj.class == "route-set" {
        let members = gather_members(&obj.body);
        route_sets.push(AsOrRouteSet::new(obj.name, obj.body, members));
        match route_sets.len() {
            l if l % 0xFF == 0 => debug!("Parsed {l} route_sets."),
            _ => (),
        }
    } else if obj.class == "filter-set" {
        // TODO: Parse filter set.
    } else if obj.class == "peering-set" {
        send_peering_set.send(obj)?;
    }
    Ok(())
}

pub fn read_db<R>(db: BufReader<R>) -> Result<Dump>
where
    R: Read,
{
    let (mut as_sets, mut route_sets) = (Vec::new(), Vec::new());
    let (mut send_aut_num, aut_num_worker) = spawn_aut_num_worker()?;
    let (mut send_peering_set, peering_set_worker) = spawn_peering_set_worker()?;

    for obj in rpsl_objects(io_wrapper_lines(db)) {
        parse_object(
            obj,
            &mut as_sets,
            &mut route_sets,
            &mut send_aut_num,
            &mut send_peering_set,
        )?;
    }

    drop((send_aut_num, send_peering_set));
    let aut_nums = aut_num_worker.join().unwrap()?;
    let peering_sets = peering_set_worker.join().unwrap()?;

    Ok(Dump {
        aut_nums,
        as_sets,
        route_sets,
        peering_sets,
    })
}
