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
            RangeEnd => unreachable!("`RangeEnd` should be handled only in `handle_permit_loose`"),
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
        let report = match self.expect_next_asn() {
            Ok(asn) => self.handle_literal_and_asn(literal, asn)?,
            Err(err) => return err,
        };
        Ok(report & self.check(walker).to_all()?)
    }

    fn handle_literal_and_asn(&self, literal: AsOrSet, asn: u64) -> AllReport {
        match literal {
            AsSet(set) => self.handle_literal_set(asn, set),
            AsNum(n) if asn == n => Ok(OkT),
            AsNum(n) => self.err(),
        }
    }

    fn handle_literal_set(&self, asn: u64, set: &str) -> AllReport {
        match self.c.set_has_member(set, asn) {
            Ok(true) => Ok(OkT),
            Ok(false) => self.err(),
            Err(skip) => skip.to_all(),
        }
    }

    fn handle_permit(&self, walker: Walker, permit: AsOrSet) -> AllReport {
        let asn = match self.expect_next_asn() {
            Ok(n) => n,
            Err(err) => return err,
        };
        self.handle_range(walker, permit, asn).to_all()
    }

    fn handle_range(
        &self,
        mut walker: Walker<'a>,
        mut permit: AsOrSet<'a>,
        asn: u64,
    ) -> AnyReport {
        let mut report = SkipF(vec![]);
        loop {
            match self.handle_literal_and_asn(permit, asn).to_any() {
                Some(new_report) => {
                    report |= new_report;
                }
                None => {
                    walker.skip_ranges();
                    return self.check(walker);
                }
            }
            match walker.next() {
                Some(Ok(Permit(p))) => permit = p,
                Some(Ok(RangeEnd)) => break,
                Some(Err(err)) => return self.handle_interpret_err(err).to_any(),
                _ => unreachable!("only `Permit` and `RangeEnd` should follow `Permit`"),
            }
        }
        match report {
            SkipF(items) if items.is_empty() => self.err().to_any(),
            skips => Some(skips | self.check(walker)?),
        }
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

    fn expect_next_asn(&self) -> Result<u64, AllReport> {
        match self.next_asn() {
            Some(Ok(n)) => Ok(n),
            Some(Err(err)) => Err(err),
            None => Err(self.err()),
        }
    }

    fn next_asn(&self) -> Option<Result<u64, AllReport>> {
        match self.path.first()? {
            AsPathEntry::Seq(n) => Some(Ok(*n)),
            _ => Some(Err(self.path_with_set())),
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
