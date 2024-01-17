use std::env::args;

use anyhow::{bail, Result};
use route_verification::*;

fn main() -> Result<()> {
    // TODO: Make a shell.
    env_logger::init();
    let args: Vec<_> = args().collect();
    if args.len() < 2 {
        bail!("Specify a command!");
    }
    match args[1].as_str() {
        "parse" => parse_one(args),
        "parse_all" => parse_all(args),
        "parse_priority" => parse_priority(args),
        "parse_ordered" => parse_ordered(args),
        "read" => read(args),
        "report" => report(args),
        other => bail!("Unknown command {other}!"),
    }
}
