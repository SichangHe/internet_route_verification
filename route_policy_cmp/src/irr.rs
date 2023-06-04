use std::{
    io::{BufReader, Read, Write},
    process::{ChildStdin, ChildStdout, Command, Stdio},
};

use crate::{
    lex::{
        dump::Dump,
        lines::{
            expressions, io_wrapper_lines, lines_continued, rpsl_objects, RPSLObject, RpslExpr,
        },
        rpsl_object::{AsOrRouteSet, AutNum},
    },
    serde::from_str,
};

use anyhow::Result;

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
    dump: &mut Dump,
    aut_num_in: &mut ChildStdin,
    aut_num_out: &mut BufReader<ChildStdout>,
) -> Result<()> {
    if obj.class == "aut-num" {
        let mut msg = serde_json::to_string(&obj)?;
        msg += "\n";
        aut_num_in.write_all(msg.as_bytes())?;
        let line = read_line_wait(aut_num_out)?;
        let aut_num: AutNum = from_str(&line)?;
        dump.aut_nums.push(aut_num);
    } else if obj.class == "as-set" {
        let members = gather_members(&obj.body);
        dump.as_sets
            .push(AsOrRouteSet::new(obj.name, obj.body, members));
    } else if obj.class == "route-set" {
        let members = gather_members(&obj.body);
        dump.route_sets
            .push(AsOrRouteSet::new(obj.name, obj.body, members));
    } else if obj.class == "peering-set" {
        // TODO: Pipe to Python.
    }
    Ok(())
}

pub fn read_db<R>(db: BufReader<R>) -> Result<Dump>
where
    R: Read,
{
    let mut py_parse_aut_num = Command::new("pypy3");
    let mut aut_num_child = py_parse_aut_num
        .args(["-m", "rpsl_policy.aut_num"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut aut_num_in = aut_num_child.stdin.take().unwrap();
    let mut aut_num_out = BufReader::new(aut_num_child.stdout.take().unwrap());

    let mut dump = Dump::default();
    for (count, obj) in rpsl_objects(io_wrapper_lines(db)).enumerate() {
        if count % 0x1000 == 0 {
            dump.log_count();
        }
        parse_object(obj, &mut dump, &mut aut_num_in, &mut aut_num_out)?;
    }

    dump.log_count();
    Ok(dump)
}
