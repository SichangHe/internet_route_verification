//! > An RPSL object is textually represented as a list of attribute-value
//! > pairs.  Each attribute-value pair is written on a separate line.  The
//! > attribute name starts at column 0, followed by character ":" and
//! > followed by the value of the attribute.  The attribute which has the
//! > same name as the object's class should be specified first.  The
//! > object's representation ends when a blank line is encountered.  An
//! > attribute's value can be split over multiple lines, by having a
//! > space, a tab or a plus ('+') character as the first character of the
//! > continuation lines.  The character "+" for line continuation allows
//! > attribute values to contain blank lines.  More spaces may optionally
//! > be used after the continuation character to increase readability.
//! > The order of attribute-value pairs is significant.
//!
//! <https://www.rfc-editor.org/rfc/rfc2622#page-6>

use std::borrow::Cow;
use std::io::{Result, *};

use lazy_regex::regex_replace_all;

use super::*;

const CONTINUATION_CHARS: [&str; 3] = [" ", "+", "\t"];

pub fn io_wrapper_lines(reader: impl BufRead) -> impl Iterator<Item = String> {
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
/// Strip and ignore comments. Ignore empty lines.
/// Lines produced are *not* followed by `\n`.
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
        let limit = self.body.len() >> 3;
        let mut buf = String::with_capacity(limit << 1);
        for line in lines_continued(self.body.lines()) {
            buf.push_str(&line);
            buf.push('\n');
            if buf.len() >= limit {
                writer.write_all(buf.as_bytes())?;
                buf.clear();
            }
        }
        buf.push('\n');
        writer.write_all(buf.as_bytes())?;
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
                if !line.starts_with('#') {
                    error!("Invalid line for start of RPSL object: `{line}`.");
                }
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

pub fn expressions<I, S>(lines: I) -> impl Iterator<Item = RpslExpr>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    lines.into_iter().filter_map(move |line| {
        let line = line.as_ref();
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
