use super::*;

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

pub enum AsOrSet<'a> {
    AsSet(&'a String),
    AsNum(&'a String),
}

pub struct Interpreter {
    sets: CharMap,
    ans: CharMap,
    parsed: HirKind,
}

impl Interpreter {
    pub fn get_char(&self, c: char) -> Result<AsOrSet, InterpreteProblem> {
        if let Some(s) = self.sets.get(c) {
            return Ok(AsOrSet::AsSet(s));
        }
        if let Some(n) = self.ans.get(c) {
            return Ok(AsOrSet::AsNum(n));
        }
        Err(InterpreteProblem::UnknownChar)
    }
}

impl<'a> IntoIterator for &'a Interpreter {
    type Item = Result<Event<'a>, InterpreteProblem>;

    type IntoIter = Walker<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Walker::new(self, &self.parsed)
    }
}

impl FromStr for Interpreter {
    type Err = InterpreteProblem;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(InterpreteProblem::Empty);
        } else if s.contains('~') {
            return Err(InterpreteProblem::HasTilde);
        }
        let mut sets = CharMap::new_from_alpha();
        let s = as_set_replace_all(s, sets.by_ref());
        let mut ans = CharMap::new(sets.next);
        let s = as_replace_all(&s, ans.by_ref());
        let s = s.replace(' ', "");
        let parsed = match Parser::new().parse(&s) {
            Ok(p) => p.into_kind(),
            Err(_) => return Err(InterpreteProblem::InvalidRegex),
        };
        Ok(Self { sets, ans, parsed })
    }
}

pub enum InterpreteProblem {
    Empty,
    HasTilde,
    InvalidRegex,
    UnknownChar,
    HadErr,
}
