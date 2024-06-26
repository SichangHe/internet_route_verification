use itertools::Itertools;
use lazy_regex::Regex;

use super::*;

pub struct AsRegex<'a> {
    pub c: &'a CheckFilter<'a>,
    pub interpreter: Interpreter,
    pub expr: &'a str,
    pub report: AnyReportCase,
}

impl<'a> AsRegex<'a> {
    pub fn check(&mut self, path: Vec<u32>) -> AnyReport {
        let converted = match self.interpreter.run(self.expr) {
            Ok(c) => c,
            Err(HasTilde) => {
                return self
                    .c
                    .skip_any_report(|| SkipAsRegexWithTilde(self.expr.into()))
            }
            Err(_) => return self.invalid_err(),
        };
        let converted_regex = match Regex::new(converted) {
            Ok(c) => c,
            Err(_) => return self.invalid_err(),
        };
        let replacements = path.iter().map(|n| self.asn_chars(*n)).collect::<Vec<_>>();
        for chars in replacements.iter().multi_cartesian_product() {
            let haystack: String = chars.into_iter().collect();
            if converted_regex.is_match(&haystack) {
                return None;
            }
        }
        match mem::take(&mut self.report) {
            BadAnyReport(_) => self.c.bad_any_report(|| MatchRegex(self.expr.into())),
            non_bad => Some(non_bad),
        }
    }

    /// chars corresponding to `asn`.
    /// Unrecorded ASNs are assigned `¿` to avoid being matched.
    pub fn asn_chars(&mut self, asn: u32) -> Vec<char> {
        let mut result: Vec<_> = self.interpreter.get_asn(asn).into_iter().collect();
        for (set, c) in self.interpreter.as_sets_with_char() {
            match self.c.set_has_member(set, asn) {
                Ok(true) => result.push(c),
                Ok(false) => (),
                Err(r) => self.report |= r.unwrap(),
            }
        }
        if asn == self.c.accept_num {
            result.push(self.interpreter.peer_as_char());
        }
        if result.is_empty() {
            vec!['¿']
        } else {
            result
        }
    }

    fn invalid_err(&self) -> AnyReport {
        self.c
            .bad_any_report(|| RpslInvalidAsRegex(self.expr.into()))
    }
}
