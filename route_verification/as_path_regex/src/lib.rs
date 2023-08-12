use std::borrow::Cow;

use lazy_regex::{regex::Replacer, regex_replace_all, Captures};

pub fn as_replace_all<R>(s: &str, replacer: R) -> Cow<str>
where
    R: Replacer,
{
    regex_replace_all!(r"(\bAS\d+\b)", s, replacer)
}

pub struct CharMap {
    pub start: u32,
    pub next: u32,
    pub char_map: Vec<String>,
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

#[cfg(test)]
mod tests;
