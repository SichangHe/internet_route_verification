use std::fs::read_dir;

use rayon::prelude::*;
use route_verification::{as_rel::AsRelDb, bgp::QueryIr, ir::Ir};

fn main() {
    let db = AsRelDb::load_bz("data/20230701.as-rel.bz2").unwrap();
    let parsed = Ir::pal_read("parsed_all").unwrap();
    let query = QueryIr::from_ir_and_as_relationship(parsed, &db);

    let rib_files = read_dir("data/ribs")
        .unwrap()
        .map(|maybe_entry| maybe_entry.unwrap().path())
        .filter(|path| path.is_file() && (path.ends_with(".gz") || path.ends_with(".bz2")))
        .collect::<Vec<_>>();
}
