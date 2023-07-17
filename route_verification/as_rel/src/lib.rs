//! This module is originally ported from
//! <https://github.com/cunha/measurements/blob/main/metadata/as-relationships/asrel.py>.
use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    str::FromStr,
};

use anyhow::{bail, Context, Result};
use bzip2::read::BzDecoder;
use hashbrown::HashMap;
use log::debug;

mod relationship;
#[cfg(test)]
mod tests;

pub use relationship::*;

use Relationship::*;

#[derive(Clone, Debug, Default)]
pub struct AsRelDB {
    source2dest: HashMap<(u64, u64), Relationship>,
}

impl AsRelDB {
    pub fn load<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        debug!("Loading AS relationship database `{path:?}`.");
        Self::do_load(path).with_context(|| format!("loading `{path:?}` for AsRelDB"))
    }

    fn do_load(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(BzDecoder::new(file));
        Self::from_maybe_lines(reader.lines())
    }

    pub fn load_bz<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        debug!("Loading AS relationship database `{path:?}`.");
        Self::do_load_bz(path).with_context(|| format!("loading `{path:?}` for AsRelDB"))
    }

    fn do_load_bz(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Self::from_maybe_lines(reader.lines())
    }

    pub fn from_lines<I, S>(lines: I) -> Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut source2dest = HashMap::new();
        for line in lines {
            add_line_to_source2dest(line.as_ref(), &mut source2dest)?;
        }
        Ok(Self::new(source2dest))
    }

    pub fn from_maybe_lines<I, S, E>(lines: I) -> Result<Self>
    where
        I: IntoIterator<Item = Result<S, E>>,
        S: AsRef<str>,
        E: Error + Sync + Send + 'static,
    {
        let mut source2dest = HashMap::new();
        for maybe_line in lines {
            let line = maybe_line?;
            add_line_to_source2dest(line.as_ref(), &mut source2dest)?;
        }
        Ok(Self::new(source2dest))
    }

    fn new(source2dest: HashMap<(u64, u64), Relationship>) -> AsRelDB {
        debug!(
            "Loaded AS relationship database with {} links.",
            source2dest.len()
        );
        Self { source2dest }
    }

    pub fn get(&self, as1: u64, as2: u64) -> Option<Relationship> {
        match self.source2dest.get(&(as1, as2)) {
            Some(rel) => Some(*rel),
            None => self.source2dest.get(&(as2, as1)).map(|rel| rel.reversed()),
        }
    }
}

fn add_line_to_source2dest(
    line: &str,
    source2dest: &mut HashMap<(u64, u64), Relationship>,
) -> Result<()> {
    if !line.starts_with('#') {
        let (key, relationship) = try_parse_as_rel(line)?;
        source2dest.insert(key, relationship);
    }
    Ok(())
}

pub fn try_parse_as_rel(line: &str) -> Result<((u64, u64), Relationship)> {
    do_try_parse_as_rel(line).with_context(|| format!("invalid AS relationship line `{line}`"))
}

fn do_try_parse_as_rel(line: &str) -> Result<((u64, u64), Relationship)> {
    let parts: Vec<_> = line.split('|').collect();
    let relationship = parts.get(2).context("wrong number of parts")?.parse()?;
    // SAFETY: The `get` above succeeded so `parts.len() >= 3`.
    let as1 = unsafe { parts.get_unchecked(0) }.parse()?;
    // SAFETY: Same as above.
    let as2 = unsafe { parts.get_unchecked(1) }.parse()?;
    Ok(((as1, as2), relationship))
}
