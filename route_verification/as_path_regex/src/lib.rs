use std::borrow::Cow;

use common_regex::*;
use thiserror::Error;

use char_map::*;

pub use interpreter::{InterpretErr, Interpreter};

pub mod char_map;
pub mod interpreter;
#[cfg(test)]
mod tests;
