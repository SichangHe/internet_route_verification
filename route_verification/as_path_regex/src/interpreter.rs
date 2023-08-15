use super::*;

#[derive(Debug)]
pub enum AsOrSet<'a> {
    AsSet(&'a String),
    AsNum(u64),
}

#[derive(Debug)]
pub struct Interpreter {
    sets: CharMap<String>,
    ans: CharMap<u64>,
    peer_as_char: char,
    has_peer_as: bool,
    expr: String,
}

impl Interpreter {
    pub fn run(&mut self, s: &str) -> Res<&str> {
        if s.contains('~') {
            return Err(InterpretErr::HasTilde);
        }
        self.sets.next = self.next();
        let s = as_set_replace_all(s, self.sets.by_ref());
        self.ans.next = self.next();
        let s = as_replace_all(&s, self.ans.by_ref());
        let replacer = self.peer_as_char.to_string();
        let expr = peer_as_replace_all(&s, replacer);
        self.has_peer_as = s != expr;
        self.expr = expr.replace(' ', "");
        Ok(&self.expr)
    }

    pub fn as_peering_char(&self) -> char {
        self.peer_as_char
    }

    pub fn has_peer_as(&self) -> bool {
        self.has_peer_as
    }

    pub fn expr(&self) -> &str {
        &self.expr
    }

    /// char corresponding to `asn`.
    pub fn get_asn(&self, asn: u64) -> Option<char> {
        let index = self.ans.char_map.iter().position(|x| *x == asn);
        // SAFETY: We found the index, which itself is from a char.
        index.map(|i| unsafe { char::from_u32_unchecked(i as u32 + self.ans.start) })
    }

    /// char corresponding to recorded AS sets.
    pub fn as_sets_with_char(&self) -> impl Iterator<Item = (&String, char)> {
        self.sets.char_map.iter().enumerate().map(|(index, set)| {
            // SAFETY: `index` must be small enough because we recorded them.
            let c = unsafe { char::from_u32_unchecked(index as u32 + self.sets.start) };
            (set, c)
        })
    }

    /// ASN or AS set corresponding to `c`.
    pub fn get_char(&self, c: char) -> Res<AsOrSet> {
        if let Some(s) = self.sets.get(c) {
            return Ok(AsOrSet::AsSet(s));
        }
        if let Some(n) = self.ans.get(c) {
            return Ok(AsOrSet::AsNum(*n));
        }
        Err(InterpretErr::UnknownChar)
    }

    pub fn start(&self) -> u32 {
        self.sets.start.min(self.ans.start)
    }

    pub fn next(&self) -> u32 {
        self.sets.next.max(self.ans.next)
    }

    pub const fn new() -> Self {
        Self::new_with_peer_as_char(PEER_AS_CHAR)
    }

    pub const fn new_with_peer_as_char(peer_as_char: char) -> Self {
        Self {
            sets: CharMap::new_from_alpha(),
            ans: CharMap::new_from_alpha(),
            peer_as_char,
            has_peer_as: false,
            expr: String::new(),
        }
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
}

pub const PEER_AS_CHAR: char = 'Å';

pub type Res<T> = Result<T, InterpretErr>;
