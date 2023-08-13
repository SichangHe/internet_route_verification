use ::lex::Call;
use parse::{Filter::*, *};

use super::*;

impl<'a> Compliance<'a> {
    pub fn check_filter(&self, filter: &'a Filter, depth: isize) -> AnyReport {
        if depth <= 0 {
            return recursion_any_report(RecurSrc::CheckFilter);
        }
        match filter {
            FilterSet(name) => self.filter_set(name, depth),
            Any => None,
            AddrPrefixSet(prefixes) => self.filter_prefixes(prefixes),
            RouteSet(name, op) => self.filter_route_set(name, *op, depth),
            AsNum(num, op) => self.filter_as_num(*num, *op),
            AsSet(name, op) => self.filter_as_set(name, *op, depth, &mut visited()),
            AsPathRE(expr) => self.filter_as_regex(expr),
            And { left, right } => self.filter_and(left, right, depth).to_any(),
            Or { left, right } => self.filter_or(left, right, depth),
            Not(filter) => self.filter_not(filter, depth),
            Group(filter) => self.check_filter(filter, depth),
            Community(community) => self.filter_community(community),
            Invalid(reason) => self.invalid_filter(reason),
        }
    }

    fn filter_set(&self, name: &str, depth: isize) -> AnyReport {
        let filter_set = match self.dump.filter_sets.get(name) {
            Some(f) => f,
            None => return self.skip_any_report(|| SkipReason::FilterSetUnrecorded(name.into())),
        };
        let mut report = SkipFBad::const_default();
        for filter in &filter_set.filters {
            report |= self.check_filter(filter, depth - 1)?;
        }
        Some(report)
    }

    fn filter_as_num(&self, num: u64, op: RangeOperator) -> AnyReport {
        let routes = match self.dump.as_routes.get(&num) {
            Some(r) => r,
            None => {
                return match self.cmp.goes_through_num(num) {
                    true => self.skip_any_report(|| SkipReason::AsRoutesUnrecorded(num)),
                    false => empty_skip_any_report(),
                }
            }
        };
        if match_ips(&self.cmp.prefix, routes, op) {
            return None;
        }
        if self.maybe_filter_customers(num, op) {
            self.special_any_report(|| ExportCustomers)
        } else {
            self.no_match_any_report(|| MatchProblem::FilterAsNum(num, op))
        }
    }

    fn maybe_filter_customers(&self, num: u64, op: RangeOperator) -> bool {
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

    fn filter_prefixes<I>(&self, prefixes: I) -> AnyReport
    where
        I: IntoIterator<Item = &'a AddrPfxRange>,
    {
        if prefixes
            .into_iter()
            .all(|prefix| !prefix.contains(&self.cmp.prefix))
        {
            self.no_match_any_report(|| MatchProblem::FilterPrefixes)
        } else {
            None
        }
    }

    fn filter_route_set(&self, name: &str, op: RangeOperator, depth: isize) -> AnyReport {
        if depth <= 0 {
            return recursion_any_report(RecurSrc::FilterRouteSet(name.into()));
        }
        let route_set = match self.dump.route_sets.get(name) {
            Some(r) => r,
            None => return self.skip_any_report(|| SkipReason::RouteSetUnrecorded(name.into())),
        };
        let mut report = SkipFBad::const_default();
        for member in &route_set.members {
            report |= self.filter_route_set_member(member, op, depth - 1)?;
        }
        if let BadF(_) = report {
            self.no_match_any_report(|| MatchProblem::FilterRouteSet(name.into()))
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
            return recursion_any_report(RecurSrc::FilterRouteSetMember(member.clone()));
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
            return failed_any_report();
        }

        if depth <= 0 {
            return recursion_any_report(RecurSrc::FilterAsSet(name.into()));
        }
        let as_set_route = match self.dump.as_set_routes.get(name) {
            Some(r) => r,
            None => return self.skip_any_report(|| SkipReason::AsSetRouteUnrecorded(name.into())),
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

        let mut report = SkipFBad::const_default();
        for set in &as_set_route.set_members {
            report |= self.filter_as_set(set, op, depth - 1, visited)?;
        }

        if !as_set_route.unrecorded_nums.is_empty() {
            report |= self.skip_any_report(|| SkipReason::AsSetRouteSomeUnrecorded(name.into()))?;
        }

        if let BadF(_) = report {
            self.no_match_any_report(|| MatchProblem::FilterAsSet(name.into(), op))
        } else {
            Some(report)
        }
    }

    fn filter_as_regex(&self, expr: &str) -> AnyReport {
        let path = self.prev_path.iter().rev();
        let path = match path
            .map(|p| match p {
                Seq(n) => Ok(*n),
                Set(_) => Err(()),
            })
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(p) => p,
            Err(_) => return self.skip_any_report(|| SkipReason::AsRegexPathWithSet),
        };
        match expr.parse::<Interpreter>() {
            Ok(interpreter) => AsRegex::new(self, interpreter, expr).check(path),
            Err(HasTilde) => self.skip_any_report(|| SkipReason::AsRegexWithTilde(expr.into())),
            Err(_) => self.bad_rpsl_any_report(|| RpslError::InvalidAsRegex(expr.into())),
        }
    }

    fn filter_and(&self, left: &'a Filter, right: &'a Filter, depth: isize) -> AllReport {
        if depth <= 0 {
            return recursion_all_report(RecurSrc::FilterAnd);
        }
        Ok(self.check_filter(left, depth - 1).to_all()?
            & self.check_filter(right, depth).to_all()?)
    }

    fn filter_or(&self, left: &'a Filter, right: &'a Filter, depth: isize) -> AnyReport {
        if depth <= 0 {
            return recursion_any_report(RecurSrc::FilterOr);
        }
        Some(self.check_filter(left, depth - 1)? | self.check_filter(right, depth)?)
    }

    fn filter_not(&self, filter: &'a Filter, depth: isize) -> AnyReport {
        if depth <= 0 {
            return recursion_any_report(RecurSrc::FilterNot);
        }
        match self.check_filter(filter, depth) {
            Some(report @ SkipF(_)) | Some(report @ MehF(_)) => {
                Some(report | self.no_match_any_report(|| MatchProblem::Filter)?)
            }
            Some(BadF(_)) => None,
            None => self.no_match_any_report(|| MatchProblem::Filter),
        }
    }

    /// We skip community checks, but this could be an enhancement.
    /// <https://github.com/SichangHe/parse_rpsl_policy/issues/16>.
    fn filter_community(&self, community: &Call) -> AnyReport {
        if self.cmp.verbosity.record_community {
            self.skip_any_report(|| SkipReason::CommunityCheckUnimplemented(community.clone()))
        } else {
            empty_skip_any_report()
        }
    }

    fn invalid_filter(&self, reason: &str) -> AnyReport {
        self.bad_rpsl_any_report(|| RpslError::InvalidFilter(reason.into()))
    }
}

pub(crate) fn visited<'a>() -> BloomHashSet<&'a str> {
    BloomHashSet::with_capacity(16384, 262144)
}
