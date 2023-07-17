//! This module is originally ported from
//! <https://github.com/cunha/measurements/blob/main/metadata/as-relationships/asrel.py>.
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    str::FromStr,
};

use anyhow::{bail, Context, Result};
use bzip2::read::BzDecoder;
use hashbrown::HashMap;
use log::debug;

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
        let mut source2dest = HashMap::new();
        let file = File::open(path)?;
        let reader = BufReader::new(BzDecoder::new(file));
        for line in reader.lines() {
            let line = line?;
            if line.starts_with('#') {
                continue;
            }
            let (key, relationship) = try_parse_as_rel(&line)?;
            source2dest.insert(key, relationship);
        }
        debug!(
            "Loaded AS relationship database with {} links.",
            source2dest.len()
        );
        Ok(Self { source2dest })
    }

    pub fn get(&self, as1: u64, as2: u64) -> Option<Relationship> {
        match self.source2dest.get(&(as1, as2)) {
            Some(rel) => Some(*rel),
            None => self.source2dest.get(&(as2, as1)).map(|rel| rel.reversed()),
        }
    }
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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Relationship {
    P2C,
    P2P,
    C2P,
}

impl FromStr for Relationship {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "-1" => P2C,
            "0" => P2P,
            "1" => C2P,
            other => bail!("invalid AS relationship `{other}`"),
        })
    }
}

impl Relationship {
    pub fn reversed(self) -> Self {
        match self {
            P2C => C2P,
            P2P => P2P,
            C2P => P2C,
        }
    }
}
