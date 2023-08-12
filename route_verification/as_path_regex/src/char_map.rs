use super::*;

pub fn as_replace_all<R>(s: &str, replacer: R) -> Cow<str>
where
    R: Replacer,
{
    regex_replace_all!(r"AS[0-9]+"i, s, replacer)
}

pub fn as_set_replace_all<R>(s: &str, replacer: R) -> Cow<str>
where
    R: Replacer,
{
    regex_replace_all!(r"(?:AS[0-9]+:)?AS-[\-\^A-Za-z0-9:]+"i, s, replacer)
}

/// A [`Replacer`] that gathers each capture it replaces in `char_map`.
#[derive(Debug)]
pub struct CharMap<T> {
    pub start: u32,
    pub next: u32,
    pub char_map: Vec<T>,
}

impl<T> CharMap<T> {
    /// Get the capture corresponding to `c`.
    pub fn get(&self, c: char) -> Option<&T> {
        self.char_map.get((c as u32 - self.start) as usize)
    }

    pub const fn new(start: u32) -> Self {
        Self::new_with_char_map(start, Vec::new())
    }

    /// Start from `Α` (Alpha).
    pub const fn new_from_alpha() -> Self {
        Self::new_with_char_map(ALPHA_CODE, Vec::new())
    }

    pub const fn new_with_char_map(start: u32, char_map: Vec<T>) -> Self {
        Self {
            start,
            next: start,
            char_map,
        }
    }
}

impl Replacer for CharMap<String> {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        self.char_map.push(caps[0].to_owned());
        // SAFETY: Number of captures is small so `self.next` is small enough.
        let c = unsafe { char::from_u32_unchecked(self.next) };
        self.next += 1;
        dst.push(c);
    }
}

impl Replacer for CharMap<u64> {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        self.char_map.push(
            caps[0][2..]
                .parse()
                .expect(r"expecting `caps[0]` to be `AS\d+`"),
        );
        // SAFETY: Number of captures is small so `self.next` is small enough.
        let c = unsafe { char::from_u32_unchecked(self.next) };
        self.next += 1;
        dst.push(c);
    }
}

pub const ALPHA_CODE: u32 = 913;
