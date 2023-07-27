use std::ops::{BitAnd, BitOr, BitOrAssign};

use ReportItem::*;

use ::lex::Call;
use parse::*;

use super::*;

use OkTBad::*;
use SkipFBad::*;

/// Report about the validity of a route, according to the RPSL.
/// Use this in an `Option`, and use `None` to indicate "ok."
///
/// Composed of a vector of [`ReportItem`]s.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Report {
    OkImport {
        from: u64,
        to: u64,
    },
    OkExport {
        from: u64,
        to: u64,
    },
    OkSingleExport {
        from: u64,
    },
    SkipImport {
        from: u64,
        to: u64,
        items: Vec<ReportItem>,
    },
    SkipExport {
        from: u64,
        to: u64,
        items: Vec<ReportItem>,
    },
    SkipSingleExport {
        from: u64,
        items: Vec<ReportItem>,
    },
    AsPathPairWithSet {
        from: AsPathEntry,
        to: AsPathEntry,
    },
    SetSingleExport {
        from: Vec<u64>,
    },
    MehImport {
        from: u64,
        to: u64,
        items: ReportItems,
    },
    MehExport {
        from: u64,
        to: u64,
        items: ReportItems,
    },
    MehSingleExport {
        from: u64,
        items: ReportItems,
    },
    BadImport {
        from: u64,
        to: u64,
        items: Vec<ReportItem>,
    },
    BadExport {
        from: u64,
        to: u64,
        items: Vec<ReportItem>,
    },
    BadSingeExport {
        from: u64,
        items: Vec<ReportItem>,
    },
}

/// Single item in [`Report`] to signal some status.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ReportItem {
    Skip(SkipReason),
    Special(SpecialCase),
    NoMatch(MatchProblem),
    BadRpsl(RpslError),
    Recursion(RecurSrc),
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SkipReason {
    FilterSetUnrecorded(String),
    AsRoutesUnrecorded(u64),
    RouteSetUnrecorded(String),
    AsSetUnrecorded(String),
    AsSetRouteUnrecorded(String),
    // TODO: Remove once implemented.
    AsRegexUnimplemented(String),
    SkippedNotFilterResult,
    CommunityCheckUnimplemented(Call),
    PeeringSetUnrecorded(String),
    SkippedExceptPeeringResult,
    AutNumUnrecorded(u64),
    ImportEmpty,
    ExportEmpty,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SpecialCase {
    Uphill,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum MatchProblem {
    Filter,
    FilterAsNum(u64, RangeOperator),
    FilterAsSet(String, RangeOperator),
    FilterPrefixes,
    FilterRouteSet(String),
    RemoteAsNum(u64),
    RemoteAsSet(String),
    ExceptPeeringRightMatch,
    Peering,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RpslError {
    InvalidAsName(String),
    InvalidFilter(String),
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RecurSrc {
    CheckFilter,
    FilterRouteSet(String),
    FilterRouteSetMember(RouteSetMember),
    FilterAsSet(String),
    FilterAsName(AsName),
    FilterAnd,
    FilterOr,
    FilterNot,
    CheckRemoteAs,
    RemoteAsName(AsName),
    RemoteAsSet(String),
    RemotePeeringSet(String),
    PeeringAnd,
    PeeringOr,
    PeeringExcept,
}

pub type ReportItems = Vec<ReportItem>;

pub trait JoinReportItems {
    fn join(self, other: Option<ReportItems>) -> Self;
}

impl JoinReportItems for Option<ReportItems> {
    fn join(self, other: Option<ReportItems>) -> Self {
        match self {
            Some(mut self_reports) => match other {
                Some(other_reports) => {
                    self_reports.extend(other_reports);
                    Some(self_reports)
                }
                None => Some(self_reports),
            },
            None => other,
        }
    }
}

/// Useful if all of the reports need to succeed.
/// - `Ok(Some(skips))` indicates skip.
/// - `Ok(None)` indicates success.
/// - `Err(errors)` indicates failure.
pub type AllReport = Result<OkTBad, ReportItems>;

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

    pub fn to_all(self) -> AllReport {
        Ok(self)
    }
}

impl BitAnd for OkTBad {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.join(rhs)
    }
}

/// Useful if any of the reports succeeding is enough.
/// - `None` indicates success.
pub type AnyReport = Option<SkipFBad>;

pub fn skip_any_report(reason: SkipReason) -> AnyReport {
    let skips = vec![Skip(reason)];
    Some(SkipF(skips))
}

pub fn skip_any_reports<I>(reasons: I) -> AnyReport
where
    I: IntoIterator<Item = SkipReason>,
{
    let skips = reasons.into_iter().map(Skip).collect();
    Some(SkipF(skips))
}

pub const fn empty_skip_any_report() -> AnyReport {
    Some(SkipF(vec![]))
}

pub fn special_any_report(reason: SpecialCase) -> AnyReport {
    let specials = vec![Special(reason)];
    Some(MehF(specials))
}

pub const fn empty_meh_any_report() -> AnyReport {
    Some(MehF(vec![]))
}

pub fn no_match_any_report(reason: MatchProblem) -> AnyReport {
    let errors = vec![NoMatch(reason)];
    Some(BadF(errors))
}

pub fn bad_rpsl_any_report(reason: RpslError) -> AnyReport {
    let errors = vec![BadRpsl(reason)];
    Some(BadF(errors))
}

pub fn recursion_any_report(reason: RecurSrc) -> AnyReport {
    let errors = vec![Recursion(reason)];
    Some(BadF(errors))
}

/// Empty failed `AnyReport`.
pub const fn failed_any_report() -> AnyReport {
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
                    SkipF(items)
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

    pub fn to_any(self) -> AnyReport {
        Some(self)
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
