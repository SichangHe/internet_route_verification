use super::*;

/// Useful if all of the reports need to succeed.
/// - `Ok(Some(skips))` indicates skip.
/// - `Ok(None)` indicates success.
/// - `Err(errors)` indicates failure.
pub type AllReport = Result<OkTBad, ReportItems>;

pub trait ToAnyReport {
    fn to_any(self) -> AnyReport;
}

impl ToAnyReport for AllReport {
    fn to_any(self) -> AnyReport {
        match self {
            Ok(OkT) => None,
            Ok(SkipT(items)) => Some(SkipF(items)),
            Ok(MehT(items)) => Some(MehF(items)),
            Err(items) => Some(BadF(items)),
        }
    }
}

pub fn skip_all_report(reason: SkipReason) -> AllReport {
    let skips = vec![Skip(reason)];
    Ok(SkipT(skips))
}

pub const fn empty_skip_all_report() -> AllReport {
    Ok(SkipT(vec![]))
}

pub fn no_match_all_report(reason: MatchProblem) -> AllReport {
    let errors = vec![NoMatch(reason)];
    Err(errors)
}

pub fn bad_rpsl_all_report(reason: RpslError) -> AllReport {
    let errors = vec![BadRpsl(reason)];
    Err(errors)
}

pub fn recursion_all_report(reason: RecurSrc) -> AllReport {
    let errors = vec![Recursion(reason)];
    Err(errors)
}

pub const fn failed_all_report() -> AllReport {
    Err(vec![])
}

pub enum OkTBad {
    OkT,
    SkipT(ReportItems),
    MehT(ReportItems),
}

impl OkTBad {
    pub fn join(self, other: Self) -> Self {
        match self {
            OkT => other,
            SkipT(mut items) => {
                match other {
                    OkT => (),
                    SkipT(i) | MehT(i) => items.extend(i),
                };
                SkipT(items)
            }
            MehT(mut items) => match other {
                OkT => MehT(items),
                SkipT(i) => {
                    items.extend(i);
                    SkipT(items)
                }
                MehT(i) => {
                    items.extend(i);
                    MehT(items)
                }
            },
        }
    }
}

impl BitAnd for OkTBad {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.join(rhs)
    }
}
