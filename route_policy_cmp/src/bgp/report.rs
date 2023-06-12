use serde::{Deserialize, Serialize};
use Report::*;
use ReportItem::*;

use crate::{
    lex::community::Call,
    parse::{address_prefix::RangeOperator, aut_sys::AsName, set::RouteSetMember},
};

use super::map::AsPathEntry;

/// Use this in an `Option`, and use `None` to indicate "good."
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Report {
    Neutral(Vec<ReportItem>),
    Bad(Vec<ReportItem>),
}

impl Report {
    pub fn skip(reason: SkipReason) -> Self {
        Neutral(vec![Skip(reason)])
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ReportItem {
    Skip(SkipReason),
    NoMatch(MatchProblem),
    BadRpsl(RpslError),
    Recursion(RecurSrc),
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SkipReason {
    FilterSetUnrecorded(String),
    AsRoutesUnrecorded(usize),
    RouteSetUnrecorded(String),
    AsSetUnrecorded(String),
    // TODO: Remote once implemented.
    AsRegexUnimplemented(String),
    SkippedNotFilterResult,
    // TODO: Remote once implemented.
    CommunityCheckUnimplemented(Call),
    PeeringSetUnrecorded(String),
    SkippedExceptPeeringResult,
    AsPathPairWithSet(AsPathEntry, AsPathEntry),
    AsPathWithSet(AsPathEntry),
    AutNumUnrecorded(usize),
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum MatchProblem {
    /// Left side should not export to right side.
    NoExportRule(usize, usize),
    NoExportRuleSingle(usize),
    /// Left side should not import from right side.
    NoImportRule(usize, usize),
    FilterAsNum(usize, RangeOperator),
    FilterAsSet(String, RangeOperator),
    FilterPrefixes,
    FilterRouteSet(String),
    NotFilterMatch,
    RemoteAsNum(usize),
    RemoteAsSet(String),
    ExceptFilterRightMatch,
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
pub type AllReport = Result<Option<ReportItems>, ReportItems>;

pub fn skip_all_report(reason: SkipReason) -> AllReport {
    let skips = vec![Skip(reason)];
    Ok(Some(skips))
}

pub fn no_match_all_report(reason: MatchProblem) -> AllReport {
    let errors = vec![NoMatch(reason)];
    Err(errors)
}

pub fn recursion_all_report(reason: RecurSrc) -> AllReport {
    let errors = vec![Recursion(reason)];
    Err(errors)
}

/// Useful if any of the reports succeeding is enough.
/// - `Some((errors, true))` indicates failure.
/// - `Some((skips, false))` indicates skip.
/// - `None` indicates success.
pub type AnyReport = Option<(ReportItems, bool)>;

pub fn skip_any_report(reason: SkipReason) -> AnyReport {
    let skips = vec![Skip(reason)];
    Some((skips, false))
}

pub fn no_match_any_report(reason: MatchProblem) -> AnyReport {
    let errors = vec![NoMatch(reason)];
    Some((errors, true))
}

pub fn bad_rpsl_any_report(reason: RpslError) -> AnyReport {
    let errors = vec![BadRpsl(reason)];
    Some((errors, true))
}

pub fn recursion_any_report(reason: RecurSrc) -> AnyReport {
    let errors = vec![Recursion(reason)];
    Some((errors, true))
}

pub fn failed_any_report() -> AnyReport {
    Some((vec![], true))
}

pub trait ToAnyReport {
    fn to_any(self) -> AnyReport;
}

impl ToAnyReport for AllReport {
    fn to_any(self) -> AnyReport {
        match self {
            Ok(Some(skips)) => Some((skips, false)),
            Ok(None) => None,
            Err(errors) => Some((errors, true)),
        }
    }
}

pub trait ToAllReport {
    fn to_all(self) -> AllReport;
}

impl ToAllReport for AnyReport {
    fn to_all(self) -> AllReport {
        match self {
            Some((errors, true)) => Err(errors),
            Some((skips, false)) => Ok(Some(skips)),
            None => Ok(None),
        }
    }
}

impl ToAllReport for Option<ReportItems> {
    fn to_all(self) -> AllReport {
        Ok(self)
    }
}

pub struct AnyReportAggregator {
    pub report_items: ReportItems,
    pub all_fail: bool,
}

impl AnyReportAggregator {
    pub fn new() -> Self {
        Self {
            report_items: vec![],
            all_fail: true,
        }
    }

    pub fn join(&mut self, (report_items, fail): (ReportItems, bool)) {
        self.report_items.extend(report_items);
        self.all_fail = self.all_fail || fail;
    }
}

impl ToAnyReport for AnyReportAggregator {
    fn to_any(self) -> AnyReport {
        Some((self.report_items, self.all_fail))
    }
}

impl Default for AnyReportAggregator {
    fn default() -> Self {
        Self::new()
    }
}

impl From<(ReportItems, bool)> for AnyReportAggregator {
    fn from((report_items, all_fail): (ReportItems, bool)) -> Self {
        Self {
            report_items,
            all_fail,
        }
    }
}
