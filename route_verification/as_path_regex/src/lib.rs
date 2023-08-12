use std::borrow::Cow;

use lazy_regex::{regex_replace_all, Captures};

pub use lazy_regex::regex::Replacer;

// TODO: Error out on `~`.
// TODO: Remove spaces.

pub fn as_replace_all<R>(s: &str, replacer: R) -> Cow<str>
where
    R: Replacer,
{
    regex_replace_all!(r"AS[0-9]+", s, replacer)
}


pub fn as_set_replace_all<R>(s: &str, replacer: R) -> Cow<str>
where
    R: Replacer,
{
    regex_replace_all!(r"(?:AS[0-9]+:)?AS-[\-\^A-Za-z0-9:]+", s, replacer)
}

/// A [`Replacer`] that gathers each capture it replaces in `char_map`.
pub struct CharMap {
    pub start: u32,
    pub next: u32,
    pub char_map: Vec<String>,
}

impl CharMap {
    /// Get the capture corresponding to `c`.
    pub fn get(&self, c: char) -> Option<&String> {
        self.char_map.get((c as u32 - self.start) as usize)
    }

    /// Start from `Α` (Alpha).
    pub const fn new_from_alpha() -> Self {
        Self::new(ALPHA_CODE, Vec::new())
    }

    pub const fn new(start: u32, char_map: Vec<String>) -> Self {
        Self {
            start,
            next: start,
            char_map,
        }
    }
}

impl Replacer for CharMap {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        self.char_map.push(caps[0].to_owned());
        // SAFETY: Number of captures is small so `self.next` is small enough.
        let c = unsafe { char::from_u32_unchecked(self.next) };
        self.next += 1;
        dst.push(c);
    }
}

pub const ALPHA_CODE: u32 = 913;

#[cfg(test)]
mod tests;
