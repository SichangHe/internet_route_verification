use std::borrow::Cow;
use std::io::{BufRead, BufReader, Read, Result, Write};
use std::mem;

use lazy_regex::regex_replace_all;
use log::error;

const CONTINUATION_CHARS: [&str; 3] = [" ", "+", "\t"];

pub fn io_wrapper_lines<R>(reader: BufReader<R>) -> impl Iterator<Item = String>
where
    R: Read,
{
    reader
        .lines()
        .filter_map(|r| r.map_err(|e| error!("{e}")).ok())
}

pub struct LinesContinued<I, S>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    lines: I,
    last_line: String,
}

impl<I, S> Iterator for LinesContinued<I, S>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        for line in self.lines.by_ref() {
            // Remove comments.
            let line = line.as_ref().split('#').next().unwrap();
            // Handle continuation lines.
            if CONTINUATION_CHARS.iter().any(|&ch| line.starts_with(ch)) {
                self.last_line.push(' ');
                self.last_line.push_str(line[1..].trim());
                continue;
            }
            // Not a continuation line.
            if !self.last_line.is_empty() {
                return Some(mem::replace(&mut self.last_line, line.trim().into()));
            }
            self.last_line = line.trim().into();
        }
        (!self.last_line.is_empty()).then(|| mem::take(&mut self.last_line))
    }
}

/// Merge continued RPSL lines in `raw_lines` into single lines according to
/// prefix continuation characters and yield them one by one.
/// Strip and ignore comments. Ignore empty lines."""
pub fn lines_continued<I, S>(raw_lines: I) -> impl Iterator<Item = String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    LinesContinued {
        lines: raw_lines.into_iter(),
        last_line: String::new(),
    }
}

pub fn dedup_whitespace(string: &str) -> Cow<str> {
    regex_replace_all!(r"\s+", string, |_| " ")
}

pub fn cleanup_right_whitespace(string: &str) -> Cow<str> {
    dedup_whitespace(string.trim_end())
}

pub fn cleanup_whitespace(string: &str) -> Cow<str> {
    dedup_whitespace(string.trim())
}

/// Objects generated using `RpslObjects` have body ending with `\n`.
pub struct RPSLObject {
    pub class: String,
    pub name: String,
    pub body: String,
}

impl RPSLObject {
    fn new(class: String, name: String, body: String) -> Self {
        Self { class, name, body }
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(self.name.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.write_all(self.body.as_bytes())?;
        writer.write_all(b"\n")?;
        Ok(())
    }
}

pub struct RpslObjects<I>
where
    I: Iterator<Item = String>,
{
    lines: I,
    class: String,
    name: String,
    body: String,
    new: bool,
}

impl<I> Iterator for RpslObjects<I>
where
    I: Iterator<Item = String>,
{
    type Item = RPSLObject;

    fn next(&mut self) -> Option<Self::Item> {
        for line in self.lines.by_ref() {
            let line = cleanup_right_whitespace(&line);
            if line.is_empty() {
                // Empty line suggests the end of the last object.
                if self.new {
                    continue;
                }
                self.new = true;
                // Yield the last object.
                return Some(RPSLObject::new(
                    mem::take(&mut self.class),
                    mem::take(&mut self.name),
                    mem::take(&mut self.body),
                ));
            }
            if !self.new {
                self.body.push_str(&line);
                self.body.push('\n');
                continue;
            }
            // Start of new object.
            if !line.contains(':') {
                error!("Invalid line for start of RPSL object: `{line}`.");
                continue;
            }
            let mut parts = line.splitn(2, ':').map(cleanup_whitespace);
            self.class = parts.next().unwrap().into();
            self.name = parts.next().unwrap().into();
            self.new = false;
            self.body.clear();
        }
        if self.new {
            None
        } else {
            // The last line is not empty.
            self.new = true;
            Some(RPSLObject::new(
                mem::take(&mut self.class),
                mem::take(&mut self.name),
                mem::take(&mut self.body),
            ))
        }
    }
}

/// Combine lines from an iterator into RPSL objects.
pub fn rpsl_objects<I>(lines: I) -> impl Iterator<Item = RPSLObject>
where
    I: IntoIterator<Item = String>,
{
    RpslObjects {
        lines: lines.into_iter(),
        class: String::new(),
        name: String::new(),
        body: String::new(),
        new: false,
    }
}

pub struct RpslExpr {
    pub key: String,
    pub expr: String,
}

pub fn expressions<I>(lines: I) -> impl Iterator<Item = RpslExpr>
where
    I: IntoIterator<Item = String>,
{
    lines.into_iter().filter_map(move |line| {
        if !line.contains(':') {
            error!("Invalid expression line: `{line}`.");
            return None;
        }
        let mut parts = line.splitn(2, ':').map(str::trim);
        let key = parts.next().unwrap().into();
        let expr = parts.next().unwrap().into();
        Some(RpslExpr { key, expr })
    })
}
