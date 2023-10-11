use std::ops::{BitAnd, BitOr, BitOrAssign};

use ::lex::Call;
use parse::*;

use super::*;

mod all;
mod any;

pub use {all::*, any::*};

use {AllReportCase::*, AnyReportCase::*, Report::*};

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
    UnrecImport {
        from: u64,
        to: u64,
        items: ReportItems,
    },
    UnrecExport {
        from: u64,
        to: u64,
        items: ReportItems,
    },
    UnrecSingleExport {
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
    BadSingleExport {
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
    // Skip unrecorded.
    UnrecordedFilterSet(String),
    UnrecordedAsRoutes(u64),
    UnrecordedRouteSet(String),
    UnrecordedAsSet(String),
    UnrecordedAsSetRoute(String),
    UnrecordedSomeAsSetRoute(String),
    UnrecordedAutNum(u64),
    UnrecordedPeeringSet(String),

    // Skip unimplemented.
    SkipAsRegexWithTilde(String),
    SkipAsRegexPathWithSet,
    SkipCommunityCheckUnimplemented(Box<Call>),

    // Skip skipped.
    SkipSkippedNotFilterResult,
    SkipSkippedExceptPeeringResult,

    // Skip empty.
    SkipImportEmpty,
    SkipExportEmpty,

    // Special case.
    /// Route from customer to provider.
    SpecUphill,
    /// Route from customer to provider that is tier-1.
    SpecUphillTier1,
    /// Export customer routes while specifying the AS itself as `<filter>`.
    SpecExportCustomers,
    /// AS in `<filter>` is the origin on the path, but the route mismatches.
    SpecAsIsOriginButNoRoute(u64),
    /// Route between Tier 1 ASes.
    SpecTier1Pair,
    /// Import route between peers while Only Imports From Providers are
    /// Specified (OIFPS).
    SpecImportPeerOIFPS,
    /// Import route from customer while Only Imports From Providers are
    /// Specified (OIFPS).
    SpecImportCustomerOIFPS,

    // Match problem.
    MatchFilter,
    MatchFilterAsNum(u64, RangeOperator),
    MatchFilterAsSet(String, RangeOperator),
    MatchFilterPrefixes,
    MatchFilterRouteSet(String),
    MatchRemoteAsNum(u64),
    MatchRemoteAsSet(String),
    MatchExceptPeeringRight,
    MatchPeering,
    MatchRegex(String),

    // Invalid RPSL.
    RpslInvalidAsName(String),
    RpslInvalidFilter(String),
    RpslInvalidAsRegex(String),
    RpslUnknownFilter(String),

    // Recursion error.
    RecCheckFilter,
    RecFilterRouteSet(String),
    RecFilterRouteSetMember(Box<RouteSetMember>),
    RecFilterAsSet(String),
    RecFilterAsName(Box<AsName>),
    RecFilterAnd,
    RecFilterOr,
    RecFilterNot,
    RecCheckSetMember(String),
    RecCheckRemoteAs,
    RecRemoteAsName(Box<AsName>),
    RecRemoteAsSet(String),
    RecRemotePeeringSet(String),
    RecPeeringAnd,
    RecPeeringOr,
    RecPeeringExcept,
}

pub type ReportItems = Vec<ReportItem>;
