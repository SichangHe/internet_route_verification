use std::env::args;

use anyhow::{bail, Result};
use env_logger::{Builder, Env};
use stat_route_objects::scan_dirs;

fn main() -> Result<()> {
    Builder::from_env(Env::default().default_filter_or("INFO")).init();
    let args: Vec<_> = args().collect();
    if args.len() < 2 {
        bail!("Specify directories separated by spaces!");
    }
    let input_dirs = &args[1..];
    scan_dirs(input_dirs)
}
