#[derive(Copy, Clone, Default)]
pub struct Counts {
    pub skip: usize,
    pub parse_err: usize,
    pub unknown_err: usize,
}

impl std::fmt::Display for Counts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            skip,
            parse_err,
            unknown_err,
        } = self;
        f.write_fmt(format_args!(
            "{skip} skips, {parse_err} parsing errors, {unknown_err} unknown errors"
        ))
    }
}
