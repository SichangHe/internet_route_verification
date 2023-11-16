use super::*;
use ReportItem::*;

pub fn one(stats: &mut RouteStats<u16>, report: Report) {
    match report {
        OkImport { from: _, to: _ } => stats.import_ok.inc(),
        OkExport { from: _, to: _ } => stats.export_ok.inc(),
        SkipImport {
            from: _,
            to: _,
            items,
        } => {
            stats.import_skip.inc();
            skip(stats, items);
        }
        SkipExport {
            from: _,
            to: _,
            items,
        } => {
            stats.export_skip.inc();
            skip(stats, items);
        }
        UnrecImport {
            from: _,
            to: _,
            items,
        } => {
            stats.import_unrec.inc();
            unrec(stats, items);
        }
        UnrecExport {
            from: _,
            to: _,
            items,
        } => {
            stats.export_unrec.inc();
            unrec(stats, items);
        }
        BadImport {
            from: _,
            to: _,
            items,
        } => {
            stats.import_err.inc();
            bad(stats, items);
        }
        BadExport {
            from: _,
            to: _,
            items,
        } => {
            stats.export_err.inc();
            bad(stats, items);
        }
        MehImport {
            from: _,
            to: _,
            items,
        } => {
            stats.import_meh.inc();
            meh(stats, items);
        }
        MehExport {
            from: _,
            to: _,
            items,
        } => {
            stats.export_meh.inc();
            meh(stats, items);
        }
        AsPathPairWithSet { from: _, to: _ } => (),
    }
}

pub fn skip<T: Inc>(stats: &mut RouteStats<T>, items: ReportItems) {
    for item in items {
        match item {
            SkipAsRegexWithTilde(_) => stats.skip_regex_tilde.inc(),
            SkipAsRegexPathWithSet => stats.skip_regex_with_set.inc(),
            SkipCommunityCheckUnimplemented(_) => stats.skip_community.inc(),
            _ => (),
        }
    }
}

pub fn unrec<T: Inc>(stats: &mut RouteStats<T>, items: ReportItems) {
    for item in items {
        match item {
            UnrecImportEmpty => stats.unrec_import_empty.inc(),
            UnrecExportEmpty => stats.unrec_export_empty.inc(),
            UnrecordedFilterSet(_) => stats.unrec_filter_set.inc(),
            UnrecordedAsRoutes(_) => stats.unrec_as_routes.inc(),
            UnrecordedRouteSet(_) => stats.unrec_route_set.inc(),
            UnrecordedAsSet(_) => stats.unrec_as_set.inc(),
            UnrecordedAsSetRoute(_) => stats.unrec_as_set_route.inc(),
            UnrecordedSomeAsSetRoute(_) => stats.unrec_some_as_set_route.inc(),
            UnrecordedAutNum(_) => stats.unrec_aut_num.inc(),
            UnrecordedPeeringSet(_) => stats.unrec_peering_set.inc(),
            _ => (),
        }
    }
}
pub fn meh<T: Inc>(stats: &mut RouteStats<T>, items: ReportItems) {
    for item in items {
        match item {
            SpecUphill => stats.spec_uphill.inc(),
            SpecUphillTier1 => stats.spec_uphill_tier1.inc(),
            SpecTier1Pair => stats.spec_tier1_pair.inc(),
            SpecImportPeerOIFPS => stats.spec_import_peer_oifps.inc(),
            SpecImportCustomerOIFPS => stats.spec_import_customer_oifps.inc(),
            SpecExportCustomers => stats.spec_export_customers.inc(),
            SpecImportFromNeighbor => stats.spec_import_from_neighbor.inc(),
            SpecAsIsOriginButNoRoute(_) => stats.spec_as_is_origin_but_no_route.inc(),
            SpecAsSetContainsOriginButNoRoute(_, _) => {
                stats.spec_as_set_contains_origin_but_no_route.inc()
            }
            _ => (),
        }
    }
}
pub fn bad<T: Inc>(stats: &mut RouteStats<T>, items: ReportItems) {
    for item in items {
        match item {
            MatchFilter => stats.err_filter.inc(),
            MatchFilterAsNum(_, _) => stats.err_filter_as_num.inc(),
            MatchFilterAsSet(_, _) => stats.err_filter_as_set.inc(),
            MatchFilterPrefixes => stats.err_filter_prefixes.inc(),
            MatchFilterRouteSet(_) => stats.err_filter_route_set.inc(),
            MatchRemoteAsNum(_) => stats.err_remote_as_num.inc(),
            MatchRemoteAsSet(_) => stats.err_remote_as_set.inc(),
            MatchExceptPeeringRight => stats.err_except_peering_right.inc(),
            MatchPeering => stats.err_peering.inc(),
            MatchRegex(_) => stats.err_regex.inc(),
            RpslInvalidAsName(_) => stats.rpsl_as_name.inc(),
            RpslInvalidFilter(_) => stats.rpsl_filter.inc(),
            RpslInvalidAsRegex(_) => stats.rpsl_regex.inc(),
            RpslUnknownFilter(_) => stats.rpsl_unknown_filter.inc(),
            RecCheckFilter
            | RecFilterRouteSet(_)
            | RecFilterRouteSetMember(_)
            | RecFilterAsSet(_)
            | RecFilterAsName(_)
            | RecFilterAnd
            | RecFilterOr
            | RecFilterNot
            | RecCheckSetMember(_)
            | RecCheckRemoteAs
            | RecRemoteAsName(_)
            | RecRemoteAsSet(_)
            | RecRemotePeeringSet(_)
            | RecPeeringAnd
            | RecPeeringOr
            | RecPeeringExcept => stats.recursion.inc(),
            _ => (),
        }
    }
}

pub trait Inc: Add + AddAssign + Copy + Default + Display + Eq + Ord + PartialOrd + Sized {
    fn inc(&mut self);
}
macro_rules! impl_inc {
    ($type: ident) => {
        impl Inc for $type {
            fn inc(&mut self) {
                *self += 1;
            }
        }
    };
}
impl_inc!(u8);
impl_inc!(u16);
impl_inc!(u32);
impl_inc!(u64);

/// Customizable integer to save space.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RouteStats<T: Inc> {
    pub import_ok: T,
    pub export_ok: T,
    pub import_skip: T,
    pub export_skip: T,
    pub import_unrec: T,
    pub export_unrec: T,
    pub import_meh: T,
    pub export_meh: T,
    pub import_err: T,
    pub export_err: T,
    pub skip_regex_tilde: T,
    pub skip_regex_with_set: T,
    pub skip_community: T,
    pub unrec_import_empty: T,
    pub unrec_export_empty: T,
    pub unrec_filter_set: T,
    pub unrec_as_routes: T,
    pub unrec_route_set: T,
    pub unrec_as_set: T,
    pub unrec_as_set_route: T,
    pub unrec_some_as_set_route: T,
    pub unrec_aut_num: T,
    pub unrec_peering_set: T,
    pub spec_uphill: T,
    pub spec_uphill_tier1: T,
    pub spec_tier1_pair: T,
    pub spec_import_peer_oifps: T,
    pub spec_import_customer_oifps: T,
    pub spec_export_customers: T,
    pub spec_import_from_neighbor: T,
    pub spec_as_is_origin_but_no_route: T,
    pub spec_as_set_contains_origin_but_no_route: T,
    pub err_filter: T,
    pub err_filter_as_num: T,
    pub err_filter_as_set: T,
    pub err_filter_prefixes: T,
    pub err_filter_route_set: T,
    pub err_remote_as_num: T,
    pub err_remote_as_set: T,
    pub err_except_peering_right: T,
    pub err_peering: T,
    pub err_regex: T,
    pub rpsl_as_name: T,
    pub rpsl_filter: T,
    pub rpsl_regex: T,
    pub rpsl_unknown_filter: T,
    pub recursion: T,
}

impl<T: Inc> RouteStats<T> {
    pub fn as_csv_bytes(&self) -> Vec<u8> {
        let Self {
            import_ok,
            export_ok,
            import_skip,
            export_skip,
            import_unrec,
            export_unrec,
            import_meh,
            export_meh,
            import_err,
            export_err,
            skip_regex_tilde,
            skip_regex_with_set,
            skip_community,
            unrec_import_empty,
            unrec_export_empty,
            unrec_filter_set,
            unrec_as_routes,
            unrec_route_set,
            unrec_as_set,
            unrec_as_set_route,
            unrec_some_as_set_route,
            unrec_aut_num,
            unrec_peering_set,
            spec_uphill,
            spec_uphill_tier1,
            spec_tier1_pair,
            spec_import_peer_oifps,
            spec_import_customer_oifps,
            spec_export_customers,
            spec_import_from_neighbor,
            spec_as_is_origin_but_no_route,
            spec_as_set_contains_origin_but_no_route,
            err_filter,
            err_filter_as_num,
            err_filter_as_set,
            err_filter_prefixes,
            err_filter_route_set,
            err_remote_as_num,
            err_remote_as_set,
            err_except_peering_right,
            err_peering,
            err_regex,
            rpsl_as_name,
            rpsl_filter,
            rpsl_regex,
            rpsl_unknown_filter,
            recursion,
        } = self;
        [
            import_ok,
            export_ok,
            import_skip,
            export_skip,
            import_unrec,
            export_unrec,
            import_meh,
            export_meh,
            import_err,
            export_err,
            skip_regex_tilde,
            skip_regex_with_set,
            skip_community,
            unrec_import_empty,
            unrec_export_empty,
            unrec_filter_set,
            unrec_as_routes,
            unrec_route_set,
            unrec_as_set,
            unrec_as_set_route,
            unrec_some_as_set_route,
            unrec_aut_num,
            unrec_peering_set,
            spec_uphill,
            spec_uphill_tier1,
            spec_tier1_pair,
            spec_import_peer_oifps,
            spec_import_customer_oifps,
            spec_export_customers,
            spec_import_from_neighbor,
            spec_as_is_origin_but_no_route,
            spec_as_set_contains_origin_but_no_route,
            err_filter,
            err_filter_as_num,
            err_filter_as_set,
            err_filter_prefixes,
            err_filter_route_set,
            err_remote_as_num,
            err_remote_as_set,
            err_except_peering_right,
            err_peering,
            err_regex,
            rpsl_as_name,
            rpsl_filter,
            rpsl_regex,
            rpsl_unknown_filter,
            recursion,
        ]
        .map(|b| b.to_string().into_bytes())
        .join(&COMMA)
    }
}

pub fn csv_header() -> String {
    "
                import_ok,
                export_ok,
                import_skip,
                export_skip,
                import_unrec,
                export_unrec,
                import_meh,
                export_meh,
                import_err,
                export_err,
                skip_regex_tilde,
                skip_regex_with_set,
                skip_community,
                unrec_import_empty,
                unrec_export_empty,
                unrec_filter_set,
                unrec_as_routes,
                unrec_route_set,
                unrec_as_set,
                unrec_as_set_route,
                unrec_some_as_set_route,
                unrec_aut_num,
                unrec_peering_set,
                spec_uphill,
                spec_uphill_tier1,
                spec_tier1_pair,
                spec_import_peer_oifps,
                spec_import_customer_oifps,
                spec_export_customers,
                spec_import_from_neighbor,
                spec_as_is_origin_but_no_route,
                spec_as_set_contains_origin_but_no_route,
                err_filter,
                err_filter_as_num,
                err_filter_as_set,
                err_filter_prefixes,
                err_filter_route_set,
                err_remote_as_num,
                err_remote_as_set,
                err_except_peering_right,
                err_peering,
                err_regex,
                rpsl_as_name,
                rpsl_filter,
                rpsl_regex,
                rpsl_unknown_filter,
                recursion,
"
    .split_ascii_whitespace()
    .collect()
}

pub const COMMA: u8 = b","[0];
