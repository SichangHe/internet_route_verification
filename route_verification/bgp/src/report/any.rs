use super::*;

/// Report generator for walking the policies of one import/export.
/// Useful if any of the reports succeeding is enough.
///
/// `None` indicates success.
/// Use `?` to always return successes early correctly.
pub type AnyReport = Option<AnyReportCase>;

pub trait ToAllReport {
    fn to_all(self) -> AllReport;
}

impl ToAllReport for AnyReport {
    fn to_all(self) -> AllReport {
        match self {
            Some(SkipAnyReport(items)) => Ok(SkipAllReport(items)),
            Some(MehAnyReport(items)) => Ok(MehAllReport(items)),
            Some(BadAnyReport(items)) => Err(items),
            None => Ok(OkAllReport),
        }
    }
}

pub fn skip_any_report(reason: ReportItem) -> AnyReport {
    let skips = vec![reason];
    Some(SkipAnyReport(skips))
}

pub fn skip_any_reports(reasons: ReportItems) -> AnyReport {
    Some(SkipAnyReport(reasons))
}

pub const fn empty_skip_any_report() -> AnyReport {
    Some(SkipAnyReport(vec![]))
}

pub fn special_any_report(reason: ReportItem) -> AnyReport {
    let specials = vec![reason];
    Some(MehAnyReport(specials))
}

pub const fn empty_meh_any_report() -> AnyReport {
    Some(MehAnyReport(vec![]))
}

pub fn bad_any_report(reason: ReportItem) -> AnyReport {
    let errors = vec![reason];
    Some(BadAnyReport(errors))
}

/// Empty failed `AnyReport`.
pub const fn empty_bad_any_report() -> AnyReport {
    Some(BadAnyReport(vec![]))
}

pub enum AnyReportCase {
    SkipAnyReport(ReportItems),
    MehAnyReport(ReportItems),
    BadAnyReport(ReportItems),
}

impl AnyReportCase {
    pub const fn const_default() -> Self {
        BadAnyReport(Vec::new())
    }

    pub fn shrink_to_fit(&mut self) {
        match self {
            SkipAnyReport(items) => items.shrink_to_fit(),
            MehAnyReport(items) => items.shrink_to_fit(),
            BadAnyReport(items) => items.shrink_to_fit(),
        }
    }
}

impl Default for AnyReportCase {
    fn default() -> Self {
        Self::const_default()
    }
}

impl BitOr for AnyReportCase {
    type Output = Self;

    /// Merge two `AnyReportCase`s based on the rule
    /// bad → meh → skip.
    fn bitor(self, other: Self) -> Self::Output {
        match (self, other) {
            (SkipAnyReport(mut items), SkipAnyReport(i) | MehAnyReport(i) | BadAnyReport(i))
            | (MehAnyReport(mut items) | BadAnyReport(mut items), SkipAnyReport(i)) => {
                items.extend(i);
                SkipAnyReport(items)
            }
            (MehAnyReport(mut items), MehAnyReport(i) | BadAnyReport(i))
            | (BadAnyReport(mut items), MehAnyReport(i)) => {
                items.extend(i);
                MehAnyReport(items)
            }

            (BadAnyReport(mut items), BadAnyReport(i)) => {
                items.extend(i);
                BadAnyReport(items)
            }
        }
    }
}

impl BitOrAssign for AnyReportCase {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = mem::take(self) | rhs;
    }
}
