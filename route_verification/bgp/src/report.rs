use std::ops::{BitAnd, BitOr, BitOrAssign};

use ReportItem::*;

use ::lex::Call;
use parse::*;

use super::*;

mod all;
mod any;

pub use {all::*, any::*};

use {OkTBad::*, Report::*, SkipFBad::*};

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
        items: ReportItems,
    },
    SkipExport {
        from: u64,
        to: u64,
        items: ReportItems,
    },
    SkipSingleExport {
        from: u64,
        items: ReportItems,
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
        items: ReportItems,
    },
    BadExport {
        from: u64,
        to: u64,
        items: ReportItems,
    },
    BadSingeExport {
        from: u64,
        items: ReportItems,
    },
}

impl Report {
    pub fn is_meh(&self) -> bool {
        matches!(
            self,
            MehImport {
                from: _,
                to: _,
                items: _
            } | MehExport {
                from: _,
                to: _,
                items: _
            } | MehSingleExport { from: _, items: _ }
        )
    }
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
    AsSetRouteSomeUnrecorded(String),
    AsRegexUnhandled(String),
    AsRegexWithTilde(String),
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
    /// Route from customer to provider.
    Uphill,
    /// Route from customer to provider that is tier-1.
    UphillTier1,
    /// Export customer routes while specifying the AS itself as `<filter>`.
    ExportCustomers,
    /// Route between Tier 1 ASes.
    Tier1Pair,
    /// Import route between peers while Only Imports From Providers are
    /// Specified (OIFPS).
    ImportPeerOIFPS,
    /// Import route from customer while Only Imports From Providers are
    /// Specified (OIFPS).
    ImportCustomerOIFPS,
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
    InvalidAsRegex(String),
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
