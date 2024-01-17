use std::{
    collections::BTreeMap,
    fmt::Display,
    io::{BufRead, BufReader, Read},
    process::ChildStdout,
    sync::mpsc::Sender,
};

use anyhow::Result;
use ir::Ir;
use lazy_regex::regex;
use lex::*;
use log::{debug, error, warn};
use parse::parse_lexed;
use rayon::prelude::*;

pub mod mbrs;
#[cfg(test)]
mod tests;
pub mod worker;

use mbrs::*;
use worker::{spawn_aut_num_worker, spawn_filter_set_worker, spawn_peering_set_worker};

/// Gather `members` and `mp-members` expressions.
/// Translate `mbrs-by-ref` expressions to pseudo sets.
pub fn gather_members(obj: &RPSLObject) -> Vec<String> {
    let mut members = Vec::new();
    for RpslExpr { key, expr } in expressions(lines_continued(obj.body.lines())) {
        match key.as_str() {
            "members" | "mp-members" => {
                members.extend(split_commas(&expr).map(Into::into));
            }
            "mbrs-by-ref" => match expr.as_str() {
                "ANY" => members.push(ref_set(&obj.name)),
                _ => members
                    .extend(split_commas(&expr).map(|mntner| mntner_ref_set(mntner, &obj.name))),
            },
            _ => (),
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

pub fn parse_object(obj: RPSLObject, pa: &mut PreAst) -> Result<()> {
    match obj.class.as_str() {
        "aut-num" => pa.send_aut_num.send(obj)?,
        "as-set" => parse_as_set(obj, &mut pa.as_sets),
        "route" | "route6" => parse_route(obj, pa),
        "route-set" => parse_route_set(obj, &mut pa.route_sets),
        "filter-set" => pa.send_filter_set.send(obj)?,
        "peering-set" => pa.send_peering_set.send(obj)?,
        _ => (),
    }
    Ok(())
}

fn parse_as_set(obj: RPSLObject, as_sets: &mut Vec<AsOrRouteSet>) {
    let members = gather_members(&obj);
    as_sets.push(AsOrRouteSet::new(obj.name, obj.body, members));
    match as_sets.len() {
        l if l % 0xFF == 0 => debug!("Parsed {l} as_sets."),
        _ => (),
    }
}

fn parse_route(obj: RPSLObject, pa: &mut PreAst) {
    gather_ref(&obj, &mut pa.pseudo_route_sets);
    for RpslExpr {
        key,
        expr, /*AS*/
    } in expressions(lines_continued(obj.body.lines()))
    {
        if key == "origin" {
            pa.as_routes
                .entry(expr.to_uppercase())
                .or_default()
                .push(obj.name /*The route*/);
            return;
        }
    }
    pa.counts.unknown_lex_err += 1;
    error!("Route object {} does not have an `origin` field.", obj.name);
}

fn parse_route_set(obj: RPSLObject, route_sets: &mut Vec<AsOrRouteSet>) {
    let members = gather_members(&obj);
    route_sets.push(AsOrRouteSet::new(obj.name, obj.body, members));
    match route_sets.len() {
        l if l % 0xFF == 0 => debug!("Parsed {l} route_sets."),
        _ => (),
    }
}

const ONE_MEBIBYTE: usize = 1024 * 1024;

/// Read and lex RPSL database.
pub fn read_db(db: impl BufRead) -> Result<(Ast, Counts)> {
    let (as_sets, route_sets, pseudo_route_sets, as_routes) =
        (Vec::new(), Vec::new(), BTreeMap::new(), BTreeMap::new());
    let (send_aut_num, aut_num_worker) = spawn_aut_num_worker()?;
    let (send_peering_set, peering_set_worker) = spawn_peering_set_worker()?;
    let (send_filter_set, filter_set_worker) = spawn_filter_set_worker()?;
    let mut pa = PreAst {
        as_sets,
        route_sets,
        pseudo_route_sets,
        send_aut_num,
        send_peering_set,
        send_filter_set,
        as_routes,
        counts: Default::default(),
    };

    for obj in rpsl_objects(io_wrapper_lines(db)) {
        if obj.body.len() > ONE_MEBIBYTE {
            // <https://github.com/SichangHe/parse_rpsl_policy/issues/6#issuecomment-1566121009>
            pa.counts.lex_skip += 1;
            warn!(
                "Skipping {} object `{}` with a {}MiB body.",
                obj.class,
                obj.name,
                obj.body.len() / ONE_MEBIBYTE
            );
            continue;
        }

        parse_object(obj, &mut pa)?;
    }
    pa.route_sets.extend(conclude_set(pa.pseudo_route_sets));

    drop((pa.send_aut_num, pa.send_peering_set, pa.send_filter_set));
    let an_out = aut_num_worker.join().unwrap()?;
    pa.as_sets.extend(an_out.pseudo_as_sets);
    let peering_sets = peering_set_worker.join().unwrap()?;
    let filter_sets = filter_set_worker.join().unwrap()?;

    let counts = pa.counts + an_out.counts;
    debug!("read_db counts: {counts}.");

    Ok((
        Ast {
            aut_nums: an_out.aut_nums,
            as_sets: pa.as_sets,
            route_sets: pa.route_sets,
            peering_sets,
            filter_sets,
            as_routes: pa.as_routes,
        },
        counts,
    ))
}

pub struct PreAst {
    pub as_sets: Vec<AsOrRouteSet>,
    pub route_sets: Vec<AsOrRouteSet>,
    pub pseudo_route_sets: Map2DStringVec,
    pub send_aut_num: Sender<RPSLObject>,
    pub send_peering_set: Sender<RPSLObject>,
    pub send_filter_set: Sender<RPSLObject>,
    pub as_routes: BTreeMap<String, Vec<String>>,
    pub counts: Counts,
}

/// Read, lex and parse a single DB.
pub fn parse_db(tag: impl Display, db: impl BufRead) -> Result<(Ir, Counts)> {
    debug!("Starting to read and lex RPSL in `{tag}`.");
    let (parsed, l_counts) = read_db(db)?;
    debug!("Starting to parse lexed `{tag}`.");
    let (ir, p_counts) = parse_lexed(parsed);
    let (n_import, n_export) = ir.aut_nums.values().fold((0, 0), |(i, e), an| {
        (i + an.imports.len(), e + an.exports.len())
    });
    debug!(
        "Read `{tag}`: {ir}; {n_import} imports, {n_export} exports. Lexing: {l_counts}. Parsing: {p_counts}.",
    );
    Ok((ir, l_counts + p_counts))
}

/// No guarantee about the priorities of the IRs.
pub fn merge_ir_and_counts<I>(ir_and_counts: I) -> (Ir, Counts)
where
    I: IntoParallelIterator<Item = (Ir, Counts)>,
{
    ir_and_counts
        .into_par_iter()
        .reduce(Default::default, |(ir_acc, counts_acc), (ir, counts)| {
            (ir_acc.merge(ir), counts_acc + counts)
        })
}

/// Priorities the IRs yielded later.
/// # Panic
/// If `ir_and_counts` is empty.
pub fn merge_ir_and_counts_ordered<I>(ir_and_counts: I) -> (Ir, Counts)
where
    I: IntoIterator<Item = (Ir, Counts)>,
{
    ir_and_counts
        .into_iter()
        .reduce(|(backup, b_counts), (priority, p_counts)| {
            (backup.merge(priority), p_counts + b_counts)
        })
        .expect("ir_and_counts should not be empty")
}

/// Split by `,`s followed by any number of whitespace.
/// Ignore empty parts.
pub fn split_commas(expr: &str) -> impl Iterator<Item = &str> {
    regex!(r",\s*").split(expr).filter_map(|s| {
        let r = s.trim();
        (!r.is_empty()).then_some(r)
    })
}

pub type Map2DStringVec = BTreeMap<String, BTreeMap<String, Vec<String>>>;
