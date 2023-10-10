use super::*;

/// Useful if all of the reports need to succeed.
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

pub fn skip_all_report(reason: ReportItem) -> AllReport {
    let skips = vec![reason];
    Ok(SkipT(skips))
}

pub const fn empty_skip_all_report() -> AllReport {
    Ok(SkipT(vec![]))
}

pub fn bad_all_report(reason: ReportItem) -> AllReport {
    let errors = vec![reason];
    Err(errors)
}

pub const fn empty_bad_all_report() -> AllReport {
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
