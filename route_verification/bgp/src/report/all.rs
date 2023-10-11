use super::*;

/// Useful if all of the reports need to succeed.
/// - `Err(errors)` indicates failure.
pub type AllReport = Result<AllReportCase, ReportItems>;

pub trait ToAnyReport {
    fn to_any(self) -> AnyReport;
}

impl ToAnyReport for AllReport {
    fn to_any(self) -> AnyReport {
        match self {
            Ok(OkAllReport) => None,
            Ok(SkipAllReport(items)) => Some(SkipAnyReport(items)),
            Ok(MehAllReport(items)) => Some(MehAnyReport(items)),
            Err(items) => Some(BadAnyReport(items)),
        }
    }
}

pub fn skip_all_report(reason: ReportItem) -> AllReport {
    let skips = vec![reason];
    Ok(SkipAllReport(skips))
}

pub const fn empty_skip_all_report() -> AllReport {
    Ok(SkipAllReport(vec![]))
}

pub fn bad_all_report(reason: ReportItem) -> AllReport {
    let errors = vec![reason];
    Err(errors)
}

pub const fn empty_bad_all_report() -> AllReport {
    Err(vec![])
}

pub enum AllReportCase {
    OkAllReport,
    SkipAllReport(ReportItems),
    MehAllReport(ReportItems),
}

impl AllReportCase {
    pub fn join(self, other: Self) -> Self {
        match self {
            OkAllReport => other,
            SkipAllReport(mut items) => {
                match other {
                    OkAllReport => (),
                    SkipAllReport(i) | MehAllReport(i) => items.extend(i),
                };
                SkipAllReport(items)
            }
            MehAllReport(mut items) => match other {
                OkAllReport => MehAllReport(items),
                SkipAllReport(i) => {
                    items.extend(i);
                    SkipAllReport(items)
                }
                MehAllReport(i) => {
                    items.extend(i);
                    MehAllReport(items)
                }
            },
        }
    }
}

impl BitAnd for AllReportCase {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.join(rhs)
    }
}
