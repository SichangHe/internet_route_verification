use std::borrow::Cow;

use lazy_regex::{regex_replace_all, Captures};
use thiserror::Error;

use char_map::*;

pub use {
    interpreter::{InterpretErr, Interpreter},
    lazy_regex::regex::Replacer,
};

pub mod char_map;
pub mod interpreter;
#[cfg(test)]
mod tests;
