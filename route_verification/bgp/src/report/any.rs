use super::*;

/// Useful if any of the reports succeeding is enough.
/// - `None` indicates success.
pub type AnyReport = Option<SkipFBad>;

pub trait ToAllReport {
    fn to_all(self) -> AllReport;
}

impl ToAllReport for AnyReport {
    fn to_all(self) -> AllReport {
        match self {
            Some(SkipF(items)) => Ok(SkipT(items)),
            Some(MehF(items)) => Ok(MehT(items)),
            Some(BadF(items)) => Err(items),
            None => Ok(OkT),
        }
    }
}

pub fn skip_any_report(reason: ReportItem) -> AnyReport {
    let skips = vec![reason];
    Some(SkipF(skips))
}

pub fn skip_any_reports(reasons: ReportItems) -> AnyReport {
    Some(SkipF(reasons))
}

pub const fn empty_skip_any_report() -> AnyReport {
    Some(SkipF(vec![]))
}

pub fn special_any_report(reason: ReportItem) -> AnyReport {
    let specials = vec![reason];
    Some(MehF(specials))
}

pub const fn empty_meh_any_report() -> AnyReport {
    Some(MehF(vec![]))
}

pub fn bad_any_report(reason: ReportItem) -> AnyReport {
    let errors = vec![reason];
    Some(BadF(errors))
}

/// Empty failed `AnyReport`.
pub const fn empty_bad_any_report() -> AnyReport {
    Some(BadF(vec![]))
}

pub enum SkipFBad {
    SkipF(ReportItems),
    MehF(ReportItems),
    BadF(ReportItems),
}

impl SkipFBad {
    pub const fn const_default() -> Self {
        BadF(Vec::new())
    }

    pub fn join(self, other: Self) -> Self {
        match self {
            SkipF(mut items) => {
                let extra = match other {
                    SkipF(i) => i,
                    MehF(i) => i,
                    BadF(i) => i,
                };
                items.extend(extra);
                SkipF(items)
            }
            MehF(mut items) => match other {
                SkipF(i) => {
                    items.extend(i);
                    SkipF(items)
                }
                MehF(i) | BadF(i) => {
                    items.extend(i);
                    MehF(items)
                }
            },
            BadF(mut items) => match other {
                SkipF(i) => {
                    items.extend(i);
                    SkipF(items)
                }
                MehF(i) => {
                    items.extend(i);
                    MehF(items)
                }
                BadF(i) => {
                    items.extend(i);
                    BadF(items)
                }
            },
        }
    }

    pub fn shrink_to_fit(&mut self) {
        match self {
            SkipF(items) => items.shrink_to_fit(),
            MehF(items) => items.shrink_to_fit(),
            BadF(items) => items.shrink_to_fit(),
        }
    }
}

impl Default for SkipFBad {
    fn default() -> Self {
        Self::const_default()
    }
}

impl BitOr for SkipFBad {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.join(rhs)
    }
}

impl BitOrAssign for SkipFBad {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = mem::take(self) | rhs;
    }
}
