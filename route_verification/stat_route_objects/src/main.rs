use std::env::args;

use anyhow::{bail, Result};
use stat_route_objects::scan_dirs;

fn main() -> Result<()> {
    env_logger::init();
    let args: Vec<_> = args().collect();
    if args.len() < 2 {
        bail!("Specify directories separated by spaces!");
    }
    let input_dirs = &args[1..];
    scan_dirs(input_dirs)
}
