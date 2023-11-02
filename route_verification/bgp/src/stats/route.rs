use super::*;
use ReportItem::*;

pub fn one(stats: &mut RouteStats, report: Report) {
    match report {
        OkImport { from: _, to: _ } => stats.import_ok += 1,
        OkExport { from: _, to: _ } | OkSingleExport { from: _ } => stats.export_ok += 1,
        SkipImport {
            from: _,
            to: _,
            items,
        } => {
            stats.import_skip += 1;
            skip(stats, items);
        }
        SkipExport {
            from: _,
            to: _,
            items,
        }
        | SkipSingleExport { from: _, items } => {
            stats.export_skip += 1;
            skip(stats, items);
        }
        UnrecImport {
            from: _,
            to: _,
            items,
        } => {
            stats.import_unrec += 1;
            unrec(stats, items);
        }
        UnrecExport {
            from: _,
            to: _,
            items,
        }
        | UnrecSingleExport { from: _, items } => {
            stats.export_unrec += 1;
            unrec(stats, items);
        }
        BadImport {
            from: _,
            to: _,
            items,
        } => {
            stats.import_err += 1;
            bad(stats, items);
        }
        BadExport {
            from: _,
            to: _,
            items,
        }
        | BadSingleExport { from: _, items } => {
            stats.export_err += 1;
            bad(stats, items);
        }
        MehImport {
            from: _,
            to: _,
            items,
        } => {
            stats.import_meh += 1;
            meh(stats, items);
        }
        MehExport {
            from: _,
            to: _,
            items,
        }
        | MehSingleExport { from: _, items } => {
            stats.export_meh += 1;
            meh(stats, items);
        }
        AsPathPairWithSet { from: _, to: _ } | SetSingleExport { from: _ } => (),
    }
}

fn skip(stats: &mut RouteStats, items: ReportItems) {
    for item in items {
        match item {
            SkipAsRegexWithTilde(_) => stats.skip_regex_tilde += 1,
            SkipAsRegexPathWithSet => stats.skip_regex_with_set += 1,
            SkipCommunityCheckUnimplemented(_) => stats.skip_community += 1,
            SkipImportEmpty => stats.skip_import_empty += 1,
            SkipExportEmpty => stats.skip_export_empty += 1,
            _ => (),
        }
    }
}

fn unrec(stats: &mut RouteStats, items: ReportItems) {
    for item in items {
        match item {
            UnrecordedFilterSet(_) => stats.unrec_filter_set += 1,
            UnrecordedAsRoutes(_) => stats.unrec_as_routes += 1,
            UnrecordedRouteSet(_) => stats.unrec_route_set += 1,
            UnrecordedAsSet(_) => stats.unrec_as_set += 1,
            UnrecordedAsSetRoute(_) => stats.unrec_as_set_route += 1,
            UnrecordedSomeAsSetRoute(_) => stats.unrec_some_as_set_route += 1,
            UnrecordedAutNum(_) => stats.unrec_aut_num += 1,
            UnrecordedPeeringSet(_) => stats.unrec_peering_set += 1,
            _ => (),
        }
    }
}
fn meh(stats: &mut RouteStats, items: ReportItems) {
    for item in items {
        match item {
            SpecUphill => stats.spec_uphill += 1,
            SpecUphillTier1 => stats.spec_uphill_tier1 += 1,
            SpecTier1Pair => stats.spec_tier1_pair += 1,
            SpecImportPeerOIFPS => stats.spec_import_peer_oifps += 1,
            SpecImportCustomerOIFPS => stats.spec_import_customer_oifps += 1,
            SpecExportCustomers => stats.spec_export_customers += 1,
            SpecImportFromNeighbor => stats.spec_import_from_neighbor += 1,
            SpecAsIsOriginButNoRoute(_) => stats.spec_as_is_origin_but_no_route += 1,
            _ => (),
        }
    }
}
fn bad(stats: &mut RouteStats, items: ReportItems) {
    for item in items {
        match item {
            MatchFilter => stats.err_filter += 1,
            MatchFilterAsNum(_, _) => stats.err_filter_as_num += 1,
            MatchFilterAsSet(_, _) => stats.err_filter_as_set += 1,
            MatchFilterPrefixes => stats.err_filter_prefixes += 1,
            MatchFilterRouteSet(_) => stats.err_filter_route_set += 1,
            MatchRemoteAsNum(_) => stats.err_remote_as_num += 1,
            MatchRemoteAsSet(_) => stats.err_remote_as_set += 1,
            MatchExceptPeeringRight => stats.err_except_peering_right += 1,
            MatchPeering => stats.err_peering += 1,
            MatchRegex(_) => stats.err_regex += 1,
            RpslInvalidAsName(_) => stats.rpsl_as_name += 1,
            RpslInvalidFilter(_) => stats.rpsl_filter += 1,
            RpslInvalidAsRegex(_) => stats.rpsl_regex += 1,
            RpslUnknownFilter(_) => stats.rpsl_unknown_filter += 1,
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
            | RecPeeringExcept => stats.recursion += 1,
            _ => (),
        }
    }
}

/// Using [u16] to save space.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RouteStats {
    pub import_ok: u16,
    pub export_ok: u16,
    pub import_skip: u16,
    pub export_skip: u16,
    pub import_unrec: u16,
    pub export_unrec: u16,
    pub import_meh: u16,
    pub export_meh: u16,
    pub import_err: u16,
    pub export_err: u16,
    pub skip_regex_tilde: u16,
    pub skip_regex_with_set: u16,
    pub skip_community: u16,
    pub skip_import_empty: u16,
    pub skip_export_empty: u16,
    pub unrec_filter_set: u16,
    pub unrec_as_routes: u16,
    pub unrec_route_set: u16,
    pub unrec_as_set: u16,
    pub unrec_as_set_route: u16,
    pub unrec_some_as_set_route: u16,
    pub unrec_aut_num: u16,
    pub unrec_peering_set: u16,
    pub spec_uphill: u16,
    pub spec_uphill_tier1: u16,
    pub spec_tier1_pair: u16,
    pub spec_import_peer_oifps: u16,
    pub spec_import_customer_oifps: u16,
    pub spec_export_customers: u16,
    pub spec_import_from_neighbor: u16,
    pub spec_as_is_origin_but_no_route: u16,
    pub err_filter: u16,
    pub err_filter_as_num: u16,
    pub err_filter_as_set: u16,
    pub err_filter_prefixes: u16,
    pub err_filter_route_set: u16,
    pub err_remote_as_num: u16,
    pub err_remote_as_set: u16,
    pub err_except_peering_right: u16,
    pub err_peering: u16,
    pub err_regex: u16,
    pub rpsl_as_name: u16,
    pub rpsl_filter: u16,
    pub rpsl_regex: u16,
    pub rpsl_unknown_filter: u16,
    pub recursion: u16,
}
