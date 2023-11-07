use std::ops::{BitAnd, BitOr, BitOrAssign};

use ::lex::Call;

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
        from: u32,
        to: u32,
    },
    OkExport {
        from: u32,
        to: u32,
    },
    OkSingleExport {
        from: u32,
    },
    SkipImport {
        from: u32,
        to: u32,
        items: ReportItems,
    },
    SkipExport {
        from: u32,
        to: u32,
        items: ReportItems,
    },
    SkipSingleExport {
        from: u32,
        items: ReportItems,
    },
    UnrecImport {
        from: u32,
        to: u32,
        items: ReportItems,
    },
    UnrecExport {
        from: u32,
        to: u32,
        items: ReportItems,
    },
    UnrecSingleExport {
        from: u32,
        items: ReportItems,
    },
    AsPathPairWithSet {
        from: AsPathEntry,
        to: AsPathEntry,
    },
    SetSingleExport {
        from: Vec<u32>,
    },
    MehImport {
        from: u32,
        to: u32,
        items: ReportItems,
    },
    MehExport {
        from: u32,
        to: u32,
        items: ReportItems,
    },
    MehSingleExport {
        from: u32,
        items: ReportItems,
    },
    BadImport {
        from: u32,
        to: u32,
        items: ReportItems,
    },
    BadExport {
        from: u32,
        to: u32,
        items: ReportItems,
    },
    BadSingleExport {
        from: u32,
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
    // Skip unimplemented.
    SkipAsRegexWithTilde(String),
    SkipAsRegexPathWithSet,
    SkipCommunityCheckUnimplemented(Box<Call>),

    // Skip empty.
    SkipImportEmpty,
    SkipExportEmpty,

    // Unrecorded.
    UnrecordedFilterSet(String),
    UnrecordedAsRoutes(u32),
    UnrecordedRouteSet(String),
    UnrecordedAsSet(String),
    UnrecordedAsSetRoute(String),
    UnrecordedSomeAsSetRoute(String),
    UnrecordedAutNum(u32),
    UnrecordedPeeringSet(String),

    // Special case for the whole import/export.
    // Unique for each import/export.
    /// Route from customer to provider.
    SpecUphill,
    /// Route from customer to provider that is tier-1.
    SpecUphillTier1,
    /// Route between Tier 1 ASes.
    SpecTier1Pair,
    /// Import route between peers while Only Imports From Providers are
    /// Specified (OIFPS).
    SpecImportPeerOIFPS,
    /// Import route from customer while Only Imports From Providers are
    /// Specified (OIFPS).
    SpecImportCustomerOIFPS,

    // Special cases for ASN filter.
    // Can be repetitive for each import/export.
    /// Export customer routes while specifying the AS itself as `<filter>`.
    SpecExportCustomers,
    /// Import from neighbor while specifying them as `<filter>`.
    SpecImportFromNeighbor,
    /// AS in `<filter>` is the origin on the path, but the route mismatches.
    SpecAsIsOriginButNoRoute(u32),
    /// AS Set in `<filter>` contains the origin AS on the path,
    /// but the route mismatches.
    SpecAsSetContainsOriginButNoRoute(String, u32),

    // Match problem.
    MatchFilter,
    MatchFilterAsNum(u32, RangeOperator),
    MatchFilterAsSet(String, RangeOperator),
    MatchFilterPrefixes,
    MatchFilterRouteSet(String),
    MatchRemoteAsNum(u32),
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
