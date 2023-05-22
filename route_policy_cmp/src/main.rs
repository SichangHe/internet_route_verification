use anyhow::Result;
use log::info;
use route_policy_cmp::{lex::dump::Dump, parse::lex::parse_lexed};
use std::fs::File;

fn main() -> Result<()> {
    env_logger::init();
    // Test lex dumped.
    info!("Loading lexed dump.");
    let file = File::open("../dump.json")?;
    let lexed = Dump::from_reader(file)?;
    info!("Loaded lexed dump.");

    // Test parse dumped.
    let parsed = parse_lexed(lexed);
    let out = File::create("../parsed.json")?;
    info!("Writing parsed dump.");
    serde_json::to_writer(out, &parsed)?;
    info!("Wrote parsed dump.");
    Ok(())
}
