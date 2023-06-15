use anyhow::{bail, Result};
use route_policy_cmp::*;

use std::env::args;

fn main() -> Result<()> {
    // TODO: Make a shell.
    env_logger::init();
    let args: Vec<_> = args().collect();
    if args.len() < 2 {
        bail!("Specify a command!");
    }
    match args[1].as_str() {
        "parse" => parse(args),
        "parse_all" => parse_all(args),
        "read" => read(args),
        other => bail!("Unknown command {other}!"),
    }
}
