use std::{
    io::{BufReader, Read},
};

use crate::lex::{
    dump::Dump,
    lines::{expressions, io_wrapper_lines, lines_continued, rpsl_objects, RPSLObject, RpslExpr},
    rpsl_object::AsOrRouteSet,
};

pub fn gather_members(body: &str) -> Vec<String> {
    let mut members = Vec::new();
    for RpslExpr { key, expr } in expressions(lines_continued(body.lines())) {
        if key == "members" || key == "mp-members" {
            members.extend(expr.split(',').map(|s| s.trim().into()));
        }
    }
    members
}

pub fn parse_object(RPSLObject { class, name, body }: RPSLObject, dump: &mut Dump) {
    if class == "aut-num" {
        // TODO: Pipe to Python.
    } else if class == "as-set" {
        let members = gather_members(&body);
        dump.as_sets.push(AsOrRouteSet::new(name, body, members));
    } else if class == "route-set" {
        let members = gather_members(&body);
        dump.route_sets.push(AsOrRouteSet::new(name, body, members));
    } else if class == "peering-set" {
        // TODO: Pipe to Python.
    }
}

pub fn read_db<R>(db: BufReader<R>) -> Dump
where
    R: Read,
{
    let mut dump = Dump::default();
    for (count, obj) in rpsl_objects(io_wrapper_lines(db)).enumerate() {
        if count % 0x1000 == 0 {
            dump.log_count();
        }
        parse_object(obj, &mut dump);
    }

    dump.log_count();
    dump
}
