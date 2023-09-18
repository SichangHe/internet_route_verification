#[derive(Copy, Clone, Default)]
pub struct Counts {
    pub skip: usize,
    pub lex_err: usize,
    pub parse_err: usize,
    pub unknown_err: usize,
}

impl std::ops::Add for Counts {
    type Output = Counts;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            skip: self.skip + rhs.skip,
            lex_err: self.lex_err + rhs.lex_err,
            parse_err: self.parse_err + rhs.parse_err,
            unknown_err: self.unknown_err + rhs.unknown_err,
        }
    }
}

impl std::fmt::Display for Counts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            skip,
            lex_err,
            parse_err,
            unknown_err,
        } = self;
        f.write_fmt(format_args!(
            "{skip} skips, {lex_err} lexing errors, {parse_err} parsing errors, {unknown_err} unknown errors"
        ))
    }
}
