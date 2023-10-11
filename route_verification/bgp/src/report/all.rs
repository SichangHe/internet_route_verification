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

impl BitAnd for AllReportCase {
    type Output = Self;

    /// Merge two `AllReportCase`s based on the rule
    /// ok → skip → meh.
    fn bitand(self, other: Self) -> Self::Output {
        match (self, other) {
            (OkAllReport, other) => other,
            (we, OkAllReport) => we,
            (MehAllReport(mut items), SkipAllReport(i) | MehAllReport(i))
            | (SkipAllReport(mut items), MehAllReport(i)) => {
                items.extend(i);
                MehAllReport(items)
            }
            (SkipAllReport(mut items), SkipAllReport(i)) => {
                items.extend(i);
                SkipAllReport(items)
            }
        }
    }
}
