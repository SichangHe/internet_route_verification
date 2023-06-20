use std::{
    io::{self, BufRead},
    mem,
    path::Path,
    process::Command,
};

use anyhow::Result;

use crate::cmd::OutputChild;

use super::{report::Report, Compare};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Line {
    pub raw: String,
    pub compare: Compare,
    pub report: Option<Vec<Report>>,
}

impl Line {
    pub fn new(raw: String, compare: Compare, report: Option<Vec<Report>>) -> Self {
        Self {
            raw,
            compare,
            report,
        }
    }

    pub fn from_raw(raw: String) -> Result<Self> {
        let compare = Compare::with_line_dump(&raw)?;
        Ok(Self::new(raw, compare, None))
    }
}

pub fn parse_mrt<P>(path: P) -> Result<Vec<Line>>
where
    P: AsRef<Path>,
{
    let output_child = read_mrt(path)?;
    pack_lines(output_child)
}

pub fn pack_lines(mut output_child: OutputChild) -> Result<Vec<Line>> {
    let mut lines = Vec::new();
    let mut line = String::new();

    while output_child.stdout.read_line(&mut line)? > 0 {
        let raw = mem::take(&mut line);
        lines.push(Line::from_raw(raw)?);
    }
    Ok(lines)
}

pub fn read_mrt<P>(path: P) -> Result<OutputChild, io::Error>
where
    P: AsRef<Path>,
{
    OutputChild::new(Command::new("bgpdump").arg("-m").arg(path.as_ref()))
}
