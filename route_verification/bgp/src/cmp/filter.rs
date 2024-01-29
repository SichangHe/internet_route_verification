use ::lex::Call;
use ir::{Filter::*, *};

use super::*;

pub struct CheckFilter<'a> {
    pub cmp: &'a Compare,
    pub query: &'a QueryIr,
    pub self_num: u32,
    pub export: bool,
    pub prev_path: &'a [AsPathEntry],
    pub mp_peerings: &'a [PeeringAction],
}

impl<'a> CheckFilter<'a> {
    pub fn check_filter(&self, filter: &'a Filter, depth: isize) -> AnyReport {
        if depth <= 0 {
            return bad_any_report(RecCheckFilter);
        }
        match filter {
            FilterSet(name) => self.filter_set(name, depth),
            Any => None,
            AddrPrefixSet(prefixes) => self.filter_prefixes(prefixes),
            RouteSet(name, op) => self.filter_route_set(name, *op, depth),
            AsNum(num, op) => self.filter_as_num(*num, *op),
            AsSet(name, op) => self.filter_as_set(name, *op),
            AsPathRE(expr) => self.filter_as_regex(expr, depth),
            And { left, right } => self.filter_and(left, right, depth).to_any(),
            Or { left, right } => self.filter_or(left, right, depth),
            Not(filter) => self.filter_not(filter, depth),
            Group(filter) => self.check_filter(filter, depth),
            Community(community) => self.filter_community(community),
            Unknown(unknown) => self.bad_any_report(|| RpslUnknownFilter(unknown.into())),
            Invalid(reason) => self.invalid_filter(reason),
        }
    }

    fn filter_set(&self, name: &str, depth: isize) -> AnyReport {
        let filter_set = match self.query.filter_sets.get(name) {
            Some(f) => f,
            None => return self.unrec_any_report(|| UnrecordedFilterSet(name.into())),
        };
        let mut report = AnyReportCase::const_default();
        for filter in &filter_set.filters {
            report |= self.check_filter(filter, depth - 1)?;
        }
        Some(report)
    }

    fn filter_as_num(&self, num: u32, op: RangeOperator) -> AnyReport {
        let routes = match self.query.as_routes.get(&num) {
            Some(r) => r,
            None => {
                return match self.cmp.goes_through_num(num) {
                    true => self.unrec_any_report(|| UnrecordedAsRoutes(num)),
                    false => empty_unrec_any_report(),
                }
            }
        };
        if match_ips(&self.cmp.prefix, routes, op) {
            return None;
        }
        if self.is_filter_export_customer(num, op) {
            self.special_any_report(|| SpecExportCustomers)
        } else if self.is_filter_as_origin(num, op) {
            self.special_any_report(|| SpecAsIsOriginButNoRoute(num))
        } else if self.is_filter_import_from_neighbor(num, op) {
            self.special_any_report(|| SpecImportFromNeighbor)
        } else {
            self.bad_any_report(|| MatchFilterAsNum(num, op))
        }
    }

    /// Check if the AS number in the `<filter>` is the origin in the AS path.
    #[inline]
    pub fn is_filter_as_origin(&self, num: u32, op: RangeOperator) -> bool {
        match self.last_on_path() {
            Some(n) => n == num && op.permits(&self.cmp.prefix),
            None => false,
        }
    }

    /// Check for this case:
    /// - The AS number itself is the `<filter>`.
    /// - Exporting customers routes.
    #[inline]
    pub fn is_filter_export_customer(&self, num: u32, op: RangeOperator) -> bool {
        if self.export && self.cmp.verbosity.check_customer && num == self.self_num {
            self.filter_as_set(&customer_set(num), op).is_none()
        } else {
            false
        }
    }

    /// Check for this case:
    /// - Importing.
    /// - The customer AS is the `<filter>`.
    /// - The `<peering>` is just the customer AS.
    /// - The prefix length matches the range operator, if any.
    #[inline]
    pub fn is_filter_import_from_neighbor(&self, num: u32, op: RangeOperator) -> bool {
        match (
            self.export,
            self.cmp.verbosity.check_customer,
            self.mp_peerings.len(),
            self.mp_peerings.first(),
        ) {
            (
                false,
                true,
                1,
                Some(PeeringAction {
                    mp_peering:
                        Peering {
                            remote_as: AsExpr::Single(AsName::Num(peering_num)),
                            remote_router: _,
                            local_router: _,
                        },
                    actions: _,
                }),
            ) => num == *peering_num && op.permits(&self.cmp.prefix),
            _ => false,
        }
    }

    /// The last AS number on the AS path.
    /// `None` if it is a set.
    pub fn last_on_path(&self) -> Option<u32> {
        match self.prev_path.last() {
            Some(Seq(n)) => Some(*n),
            None => Some(self.self_num),
            _ => None,
        }
    }

    fn filter_prefixes<I>(&self, prefixes: I) -> AnyReport
    where
        I: IntoIterator<Item = &'a AddrPfxRange>,
    {
        if prefixes
            .into_iter()
            .all(|prefix| !prefix.contains(&self.cmp.prefix))
        {
            self.bad_any_report(|| MatchFilterPrefixes)
        } else {
            None
        }
    }

    fn filter_route_set(&self, name: &str, op: RangeOperator, depth: isize) -> AnyReport {
        if depth <= 0 {
            return bad_any_report(RecFilterRouteSet(name.into()));
        }
        let route_set = match self.query.route_sets.get(name) {
            Some(r) => r,
            None => return self.unrec_any_report(|| UnrecordedRouteSet(name.into())),
        };
        let mut report = AnyReportCase::const_default();
        for member in &route_set.members {
            report |= self.filter_route_set_member(member, op, depth - 1)?;
        }
        if let BadAnyReport(_) = report {
            self.bad_any_report(|| MatchFilterRouteSet(name.into()))
        } else {
            Some(report)
        }
    }

    fn filter_route_set_member(
        &self,
        member: &RouteSetMember,
        op: RangeOperator,
        depth: isize,
    ) -> AnyReport {
        if depth <= 0 {
            return bad_any_report(RecFilterRouteSetMember(Box::new(member.clone())));
        }
        match member {
            RouteSetMember::RSRange(prefix) => match (prefix.range_operator, op) {
                (RangeOperator::NoOp, RangeOperator::NoOp) => self.filter_prefixes([prefix]),
                (RangeOperator::NoOp, op) => self.filter_prefixes([&AddrPfxRange {
                    range_operator: op,
                    ..prefix.clone()
                }]),
                _ => self.filter_prefixes([prefix]),
            },
            RouteSetMember::NameOp(name, op) => self.filter_route_set(name, *op, depth - 1),
        }
    }

    fn filter_as_set(&self, name: &'a str, op: RangeOperator) -> AnyReport {
        let as_set = match self.query.as_sets.get(name) {
            Some(s) => s,
            None => return self.unrec_any_report(|| UnrecordedAsSetRoute(name.into())),
        };

        let mut all_members_recorded = true;
        for num in &as_set.members {
            match self.query.as_routes.get(num) {
                Some(as_routes) => {
                    if match_ips(&self.cmp.prefix, as_routes, op) {
                        return None;
                    }
                }
                None => all_members_recorded = false,
            }
        }

        if all_members_recorded && as_set.unrecorded_members.is_empty() {
            match self.is_filter_as_set_origin(name, op) {
                spec_report @ Some(_) => spec_report,
                None => self.bad_any_report(|| MatchFilterAsSet(name.into(), op)),
            }
        } else {
            self.unrec_any_report(|| UnrecordedSomeAsSetRoute(name.into()))
        }
    }

    /// Same as `is_filter_as_origin` but for each member in `as_set` and
    /// returns the special case [`AnyReport`].
    #[inline]
    fn is_filter_as_set_origin(&self, as_set: &str, op: RangeOperator) -> AnyReport {
        let last = self.last_on_path()?;
        if let (true, Ok(true)) = (
            op.permits(&self.cmp.prefix),
            self.set_has_member(as_set, last),
        ) {
            self.special_any_report(|| SpecAsSetContainsOriginButNoRoute(as_set.into(), last))
        } else {
            None
        }
    }

    /// <https://www.rfc-editor.org/rfc/rfc2622#page-19>.
    fn filter_as_regex(&self, expr: &str, depth: isize) -> AnyReport {
        let path = self.prev_path.iter();
        let path = match path
            .map(|p| match p {
                Seq(n) => Ok(*n),
                Set(_) => Err(()),
            })
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(p) => p,
            Err(_) => return self.skip_any_report(|| SkipAsRegexPathWithSet),
        };
        AsRegex {
            c: self,
            interpreter: Interpreter::new(),
            expr,
            report: BadAnyReport(vec![]),
        }
        .check(path, depth)
    }

    fn filter_and(&self, left: &'a Filter, right: &'a Filter, depth: isize) -> AllReport {
        if depth <= 0 {
            return bad_all_report(RecFilterAnd);
        }
        Ok(self.check_filter(left, depth - 1).to_all()?
            & self.check_filter(right, depth).to_all()?)
    }

    fn filter_or(&self, left: &'a Filter, right: &'a Filter, depth: isize) -> AnyReport {
        if depth <= 0 {
            return bad_any_report(RecFilterOr);
        }
        Some(self.check_filter(left, depth - 1)? | self.check_filter(right, depth)?)
    }

    fn filter_not(&self, filter: &'a Filter, depth: isize) -> AnyReport {
        if depth <= 0 {
            return bad_any_report(RecFilterNot);
        }
        match self.check_filter(filter, depth) {
            Some(report @ SkipAnyReport(_) | report @ UnrecAnyReport(_)) => {
                Some(report | self.bad_any_report(|| MatchFilter)?)
            }
            Some(MehAnyReport(_) | BadAnyReport(_)) => None,
            None => self.bad_any_report(|| MatchFilter),
        }
    }

    /// We skip community checks, but this could be an enhancement.
    /// <https://github.com/SichangHe/parse_rpsl_policy/issues/16>.
    fn filter_community(&self, community: &Call) -> AnyReport {
        if self.cmp.verbosity.record_community {
            self.skip_any_report(|| SkipCommunityCheckUnimplemented(Box::new(community.clone())))
        } else {
            empty_skip_any_report()
        }
    }

    fn invalid_filter(&self, reason: &str) -> AnyReport {
        self.bad_any_report(|| RpslInvalidFilter(reason.into()))
    }

    /// `Err` contains all the skips in an [`AnyReport`].
    pub fn set_has_member(&self, set: &'a str, asn: u32) -> Result<bool, AnyReport> {
        let as_set = match self.query.as_sets.get(set) {
            Some(s) => s,
            None => return Err(self.unrec_any_report(|| UnrecordedAsSet(set.into()))),
        };
        if as_set.contains(&asn) {
            Ok(true)
        } else if !as_set.unrecorded_members.is_empty() {
            Err(self.unrec_any_report(|| UnrecordedSomeAsSet(set.into())))
        } else {
            Ok(false)
        }
    }
}

impl<'a> VerbosityReport for CheckFilter<'a> {
    fn get_verbosity(&self) -> Verbosity {
        self.cmp.verbosity
    }
}
