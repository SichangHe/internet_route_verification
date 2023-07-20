//! This module is originally ported from
//! <https://github.com/cunha/measurements/blob/main/metadata/as-relationships/asrel.py>:
//! > AS relationships
//! >
//! > This module loads CAIDA's AS relationship database and provides easy
//! > access functions.  CAIDA's database contains provider-customers and
//! > peer-peer links (but no sibling links).  At the moment of this
//! > writing, you can check CAIDA's AS relationship dataset at [here].
//! >
//! > [here]: http://data.caida.org/datasets/2013-asrank-data-supplement/
//!
//! See [`AsRelDb`] for usage.
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

/// # Autonomous System relationship database
/// **Query** [`Relationship`] between two ASes using [`get`](#method.get).
///
/// **Construct** from `.bz2` file using [`load_bz`](#method.load_bz).
/// Construct from plain text file using [`load`](#method.load).
///
/// See the **expected format** at [`from_lines`](#method.from_lines).
#[derive(Clone, Debug, Default)]
pub struct AsRelDb {
    pub source2dest: HashMap<(u64, u64), Relationship>,
}

impl AsRelDb {
    /// Load from plain text file `path`.
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
        let reader = BufReader::new(file);
        Self::from_maybe_lines(reader.lines())
    }

    /// Load from `.bz2` file `path`.
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
        let reader = BufReader::new(BzDecoder::new(file));
        Self::from_maybe_lines(reader.lines())
    }

    /// Construct from `lines` of a plain text file.
    /// # Errors
    /// One of the lines is not in the expected format.
    /// # Expected format
    /// Lines prefixed with `#` are ignored.
    ///
    /// Each line is `as1|as2|relationship` where `relationship` is among
    /// `0`, `-1`, or `1`.
    /// Though, `1` is not used in practice.
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

    /// Same as [`from_lines`](#method.from_lines) but `maybe_lines` can contain
    /// errors, in which case the error is returned.
    pub fn from_maybe_lines<I, S, E>(maybe_lines: I) -> Result<Self>
    where
        I: IntoIterator<Item = Result<S, E>>,
        S: AsRef<str>,
        E: Error + Sync + Send + 'static,
    {
        let mut source2dest = HashMap::new();
        for maybe_line in maybe_lines {
            let line = maybe_line?;
            add_line_to_source2dest(line.as_ref(), &mut source2dest)?;
        }
        Ok(Self::new(source2dest))
    }

    fn new(source2dest: HashMap<(u64, u64), Relationship>) -> AsRelDb {
        debug!(
            "Loaded AS relationship database with {} links.",
            source2dest.len()
        );
        Self { source2dest }
    }

    /// Get [`Relationship`] between `as1` and `as2`, if recorded.
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

/// Try parsing `line` as a non-comment line in a AS relationship file.
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
