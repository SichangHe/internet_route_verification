use as_path_regex::{
    interpreter::{
        AsOrSet::{self, *},
        Event::{self, *},
        InterpretErr::{self, *},
        Interpreter,
    },
    Walker,
};

use super::*;

impl<'a> Compliance<'a> {
    pub fn filter_as_regex(&self, expr: &str) -> AnyReport {
        let path = self.prev_path;
        let interpreter: Interpreter = match expr.parse() {
            Ok(i) => i,
            Err(err) => {
                return match err {
                    HasTilde => self.skip_any_report(|| SkipReason::AsRegexWithTilde(expr.into())),
                    _ => self.bad_rpsl_any_report(|| RpslError::InvalidAsRegex(expr.into())),
                }
            }
        };
        AsRegex::new(self, expr, path).check(interpreter.into_iter())
    }
}

#[derive(Clone)]
pub struct AsRegex<'a> {
    pub c: &'a Compliance<'a>,
    pub expr: &'a str,
    pub path: &'a [AsPathEntry],
}

#[allow(unused_variables)]
impl<'a> AsRegex<'a> {
    pub fn check(&self, mut walker: Walker) -> AnyReport {
        let next = walker.next()?; // Empty regex matches anything.
        self.check_next(walker, next).to_any()
    }

    pub fn check_next(&self, walker: Walker, next: Result<Event<'a>, InterpretErr>) -> AllReport {
        let next = match next {
            Ok(n) => n,
            Err(err) => return self.handle_interpret_err(err),
        };
        match next {
            Literal(literal) => self.handle_literal(walker, literal),
            Permit(permit) => self.handle_permit(walker, permit),
            Start => self.handle_start(walker),
            End => self.handle_end(walker),
            Repeat {
                min,
                max,
                greedy,
                walker: new_walker,
            } => self.handle_repeat(walker, new_walker, min, max, greedy),
            Or(or_walker) => self.handle_or(walker, or_walker),
        }
    }

    fn handle_literal(&self, walker: Walker, literal: AsOrSet) -> AllReport {
        let asn = match self.next_asn() {
            Some(Some(n)) => n,
            Some(None) => return self.path_with_set(),
            None => return self.err(),
        };
        match literal {
            AsSet(_) => todo!(),
            AsNum(_) => todo!(),
        }
    }

    fn handle_permit(&self, walker: Walker, permit: AsOrSet) -> AllReport {
        todo!()
    }

    fn handle_start(&self, walker: Walker) -> AllReport {
        todo!()
    }

    fn handle_end(&self, walker: Walker) -> AllReport {
        todo!()
    }

    fn handle_repeat(
        &self,
        walker: Walker,
        new_walker: Walker,
        min: u32,
        max: Option<u32>,
        greedy: bool,
    ) -> AllReport {
        todo!()
    }

    fn handle_or(&self, walker: Walker, or_walker: Walker) -> AllReport {
        todo!()
    }

    /// The first layer of `Option` indicates if the path ends.
    /// The second layer indicates if the path is a single ASN.
    fn next_asn(&self) -> Option<Option<u64>> {
        match self.path.first()? {
            AsPathEntry::Seq(n) => Some(Some(*n)),
            _ => Some(None),
        }
    }

    fn handle_interpret_err(&self, err: InterpretErr) -> AllReport {
        match err {
            HasTilde => unreachable!("We checked `~` before"),
            InvalidRegex => self.regex_err(),
            UnknownChar | HadErr => self.unhandled(),
        }
    }

    fn regex_err(&self) -> AllReport {
        self.c
            .bad_rpsl_all_report(|| RpslError::InvalidAsRegex(self.expr()))
    }

    fn unhandled(&self) -> AllReport {
        self.c
            .skip_all_report(|| SkipReason::AsRegexUnhandled(self.expr()))
    }

    fn path_with_set(&self) -> AllReport {
        self.c.skip_all_report(|| SkipReason::AsRegexPathWithSet)
    }

    fn err(&self) -> AllReport {
        self.c
            .no_match_all_report(|| MatchProblem::RegexMismatch(self.expr()))
    }

    pub fn expr(&self) -> String {
        self.expr.into()
    }

    pub fn new(compliance: &'a Compliance<'a>, expr: &'a str, as_path: &'a [AsPathEntry]) -> Self {
        Self {
            c: compliance,
            expr,
            path: as_path,
        }
    }
}
