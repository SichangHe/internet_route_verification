use itertools::Itertools;
use lazy_regex::Regex;
use parse::filter::peer_as_filter;

use super::*;

pub struct AsRegex<'a> {
    pub c: &'a CheckFilter<'a>,
    pub interpreter: Interpreter,
    pub expr: &'a str,
    pub report: SkipFBad,
}

impl<'a> AsRegex<'a> {
    pub fn check(&mut self, path: Vec<u64>, depth: isize) -> AnyReport {
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
        let peer_as_filter = self
            .interpreter
            .has_peer_as()
            .then(|| peer_as_filter(self.c.mp_peerings, &mut Default::default()));
        let replacements: Vec<_> = path
            .iter()
            .map(|n| self.asn_chars(*n, peer_as_filter.as_ref(), depth - 1))
            .collect();
        for chars in replacements.iter().multi_cartesian_product() {
            let haystack: String = chars.into_iter().collect();
            if converted_regex.is_match(&haystack) {
                return None;
            }
        }
        match mem::take(&mut self.report) {
            BadF(_) => self
                .c
                .no_match_any_report(|| MatchRegexMismatch(self.expr.into())),
            non_bad => Some(non_bad),
        }
    }

    /// chars corresponding to `asn`.
    /// Unrecorded ASNs are assigned `¿` to avoid being matched.
    pub fn asn_chars(&mut self, asn: u64, filter: Option<&Filter>, depth: isize) -> Vec<char> {
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
        if let Some(filter) = filter {
            match self.c.check_filter(filter, depth) {
                Some(skips @ SkipF(_)) => self.report |= skips,
                Some(_) => (),
                None => result.push(self.interpreter.as_peering_char()),
            }
        }
        if result.is_empty() {
            vec!['¿']
        } else {
            result
        }
    }

    fn invalid_err(&self) -> AnyReport {
        self.c
            .bad_rpsl_any_report(|| RpslInvalidAsRegex(self.expr.into()))
    }
}
