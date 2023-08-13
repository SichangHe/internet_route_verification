use itertools::Itertools;

use super::*;

pub struct AsRegex<'a> {
    pub c: &'a Compliance<'a>,
    pub interpreter: Interpreter,
    pub expr: &'a str,
    pub report: SkipFBad,
}

impl<'a> AsRegex<'a> {
    pub fn check(&mut self, path: Vec<u64>) -> AnyReport {
        let replacements: Vec<_> = path.iter().map(|n| self.asn_chars(*n)).collect();
        for chars in replacements.iter().multi_cartesian_product() {
            let haystack: String = chars.into_iter().collect();
            if self.interpreter.regex().is_match(&haystack) {
                return None;
            }
        }
        match mem::take(&mut self.report) {
            BadF(_) => self
                .c
                .no_match_any_report(|| MatchProblem::RegexMismatch(self.expr.into())),
            non_bad => Some(non_bad),
        }
    }

    /// chars corresponding to `asn`.
    /// Unrecorded ASNs are assigned `X` to avoid being matched.
    pub fn asn_chars(&mut self, asn: u64) -> Vec<char> {
        let mut result: Vec<_> = self.interpreter.get_asn(asn).into_iter().collect();
        let limit = self.c.cmp.recursion_limit;
        let mut visited = visited();
        for (set, c) in self.interpreter.as_sets_with_char() {
            match self.c.set_has_member(set, asn, limit, &mut visited) {
                Ok(true) => result.push(c),
                Ok(false) => (),
                Err(r) => self.report |= r.unwrap(),
            }
        }
        if result.is_empty() {
            vec!['X']
        } else {
            result
        }
    }

    pub fn new(compliance: &'a Compliance<'a>, interpreter: Interpreter, expr: &'a str) -> Self {
        Self {
            c: compliance,
            interpreter,
            expr,
            report: BadF(vec![]),
        }
    }
}
