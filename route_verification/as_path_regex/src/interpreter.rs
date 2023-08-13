use super::*;

#[derive(Debug)]
pub enum Event<'a> {
    Literal(AsOrSet<'a>),
    Permit(AsOrSet<'a>),
    Start,
    End,
    Repeat {
        min: u32,
        max: Option<u32>,
        greedy: bool,
        walker: Walker<'a>,
    },
    Or(Walker<'a>),
}

#[derive(Debug)]
pub enum AsOrSet<'a> {
    AsSet(&'a String),
    AsNum(u64),
}

#[derive(Debug)]
pub struct Interpreter {
    sets: CharMap<String>,
    ans: CharMap<u64>,
    parsed: HirKind,
}

impl Interpreter {
    pub fn get_char(&self, c: char) -> Result<AsOrSet, InterpretErr> {
        if let Some(s) = self.sets.get(c) {
            return Ok(AsOrSet::AsSet(s));
        }
        if let Some(n) = self.ans.get(c) {
            return Ok(AsOrSet::AsNum(*n));
        }
        Err(InterpretErr::UnknownChar)
    }

    pub fn least_char(&self) -> char {
        // SAFETY: This interpreter is fine from `from_str`.
        unsafe { char::from_u32_unchecked(self.sets.start) }
    }

    pub fn largest_char(&self) -> char {
        // SAFETY: This interpreter is fine from `from_str`.
        unsafe { char::from_u32_unchecked(self.ans.next - 1) }
    }
}

impl<'a> IntoIterator for &'a Interpreter {
    type Item = Result<Event<'a>, InterpretErr>;

    type IntoIter = Walker<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Walker::new(self, &self.parsed)
    }
}

impl FromStr for Interpreter {
    type Err = InterpretErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('~') {
            return Err(InterpretErr::HasTilde);
        }
        let mut sets = CharMap::new_from_alpha();
        let s = as_set_replace_all(s, sets.by_ref());
        let mut ans = CharMap::new(sets.next);
        let s = as_replace_all(&s, ans.by_ref());
        let s = s.replace(' ', "");
        let parsed = match Parser::new().parse(&s) {
            Ok(p) => p.into_kind(),
            Err(_) => return Err(InterpretErr::InvalidRegex),
        };
        Ok(Self { sets, ans, parsed })
    }
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum InterpretErr {
    #[error("tilde found, unsupported")]
    HasTilde,
    #[error("invalid regex")]
    InvalidRegex,
    #[error("encountered unknown character")]
    UnknownChar,
    #[error("this `Walker` had error before and should not be used")]
    HadErr,
}
