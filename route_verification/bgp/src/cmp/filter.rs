use ::lex::Call;
use parse::{Filter::*, *};

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
            AsSet(name, op) => self.filter_as_set(name, *op, depth, &mut visited()),
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
        if self.maybe_filter_customers(num, op) {
            self.special_any_report(|| SpecExportCustomers)
        } else if self.maybe_filter_as_is_origin(num, op) {
            self.special_any_report(|| SpecAsIsOriginButNoRoute(num))
        } else {
            self.bad_any_report(|| MatchFilterAsNum(num, op))
        }
    }

    /// Check if the AS number in the `<filter>` is the origin in the AS path.
    pub fn maybe_filter_as_is_origin(&self, num: u32, op: RangeOperator) -> bool {
        match (op, self.last_on_path()) {
            (RangeOperator::NoOp, Some(n)) => n == num,
            _ => false,
        }
    }

    /// Check for this case:
    /// - The AS number itself is the `<filter>`.
    /// - Exporting customers routes.
    pub fn maybe_filter_customers(&self, num: u32, op: RangeOperator) -> bool {
        if self.export && self.cmp.verbosity.check_customer && num == self.self_num {
            self.filter_as_set(
                &customer_set(num),
                op,
                self.cmp.recursion_limit,
                &mut visited(),
            )
            .is_none()
        } else {
            false
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

    fn filter_as_set(
        &self,
        name: &'a str,
        op: RangeOperator,
        depth: isize,
        visited: &mut BloomHashSet<&'a str>,
    ) -> AnyReport {
        let hash = visited.make_hash(&name);
        if visited.contains_with_hash(&name, hash) {
            return empty_bad_any_report();
        }

        if depth <= 0 {
            return bad_any_report(RecFilterAsSet(name.into()));
        }
        let as_set_route = match self.query.as_set_routes.get(name) {
            Some(r) => r,
            None => return self.unrec_any_report(|| UnrecordedAsSetRoute(name.into())),
        };

        if match_ips(&self.cmp.prefix, &as_set_route.routes, op) {
            return None;
        }

        self.filter_as_set_members(name, op, depth, visited, hash, as_set_route)
    }

    fn filter_as_set_members(
        &self,
        name: &'a str,
        op: RangeOperator,
        depth: isize,
        visited: &mut BloomHashSet<&'a str>,
        hash: u64,
        as_set_route: &'a AsSetRoute,
    ) -> AnyReport {
        visited.insert_with_hash(name, hash);

        let mut report = AnyReportCase::const_default();
        for set in &as_set_route.set_members {
            report |= self.filter_as_set(set, op, depth - 1, visited)?;
        }

        if !as_set_route.unrecorded_nums.is_empty() {
            report |= self.unrec_any_report(|| UnrecordedSomeAsSetRoute(name.into()))?;
        }

        self.maybe_filter_as_set_is_origin(&mut report, as_set_route);

        if let BadAnyReport(_) = report {
            self.bad_any_report(|| MatchFilterAsSet(name.into(), op))
        } else {
            Some(report)
        }
    }

    /// Same as `maybe_filter_as_is_origin` but for each member in `as_set_route`.
    fn maybe_filter_as_set_is_origin(
        &self,
        report: &mut AnyReportCase,
        as_set_route: &'a AsSetRoute,
    ) {
        if let Some(last) = self.last_on_path() {
            if as_set_route.contains_member(last) {
                *report |= self
                    .special_any_report(|| SpecAsIsOriginButNoRoute(last))
                    .expect("special_any_report never returns None");
            }
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

    /// `Err` contains all the skips.
    pub fn set_has_member(
        &self,
        set: &'a str,
        asn: u32,
        depth: isize,
        visited: &mut BloomHashSet<&'a str>,
    ) -> Result<bool, AnyReport> {
        if depth < 0 {
            return Err(bad_any_report(RecCheckSetMember(set.into())));
        }
        let hash = visited.make_hash(&set);
        if visited.contains_with_hash(&set, hash) {
            return Err(empty_bad_any_report());
        }
        let as_set = match self.query.as_sets.get(set) {
            Some(s) => s,
            None => return Err(self.unrec_any_report(|| UnrecordedAsSet(set.into()))),
        };
        if as_set.is_any || as_set.members.contains(&asn) {
            return Ok(true);
        }
        let mut report = SkipAnyReport(vec![]);
        visited.insert_with_hash(set, hash);
        for set in &as_set.set_members {
            match self.set_has_member(set, asn, depth - 1, visited) {
                Ok(true) => return Ok(true),
                Ok(false) => (),
                Err(err) => report |= err.unwrap(),
            }
        }
        match report {
            SkipAnyReport(items) if items.is_empty() => Ok(false),
            report => Err(Some(report)),
        }
    }
}

impl<'a> VerbosityReport for CheckFilter<'a> {
    fn get_verbosity(&self) -> Verbosity {
        self.cmp.verbosity
    }
}

pub(crate) fn visited<'a>() -> BloomHashSet<&'a str> {
    BloomHashSet::with_capacity(16384, 262144)
}
