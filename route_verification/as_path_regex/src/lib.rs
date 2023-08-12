use std::{borrow::Cow, fmt::Display, str::FromStr};

use anyhow::Result;
use lazy_regex::{regex_replace_all, Captures};
use regex_syntax::{hir::*, Parser};
use thiserror::Error;

use {
    char_map::*,
    interpreter::{Event, InterpretErr, Interpreter},
};

pub use {lazy_regex::regex::Replacer, walker::Walker};

pub mod char_map;
pub mod interpreter;
#[cfg(test)]
mod tests;
pub mod walker;
