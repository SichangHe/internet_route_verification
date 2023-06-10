use crate::{
    lex::community::Call,
    parse::{
        address_prefix::{AddrPfxRange, RangeOperator},
        aut_sys::AsName,
        filter::Filter::{self, *},
        set::RouteSetMember,
    },
};

use super::{
    cmp::Compare,
    report::{ReportItem::*, *},
};

pub struct CheckFilter<'a> {
    pub compare: &'a Compare<'a>,
    pub accept_num: usize,
}

impl<'a> CheckFilter<'a> {
    pub fn check(&self, filter: &'a Filter, depth: isize) -> AnyReport {
        if depth <= 0 {
            return recursion_any_report(RecurSrc::CheckFilter);
        }
        match filter {
            FilterSet(name) => self.filter_set(name, depth),
            Any => None,
            AddrPrefixSet(prefixes) => self.filter_prefixes(prefixes),
            RouteSet(name, op) => self.filter_route_set(name, op, depth),
            AsNum(num, op) => self.filter_as_num(*num, op),
            AsSet(name, op) => self.filter_as_set(name, op, depth, &mut Vec::new()),
            AsPathRE(expr) => self.filter_as_regex(expr),
            And { left, right } => self.filter_and(left, right, depth).to_any(),
            Or { left, right } => self.filter_or(left, right, depth),
            Not(filter) => self.filter_not(filter, depth),
            Group(filter) => self.check(filter, depth),
            Community(community) => self.filter_community(community),
            Invalid(reason) => self.invalid_filter(reason),
        }
    }

    fn filter_set(&self, name: &str, depth: isize) -> AnyReport {
        let filter_set = match self.compare.dump.filter_sets.get(name) {
            Some(f) => f,
            None => return skip_any_report(SkipReason::FilterSetUnrecorded(name.into())),
        };
        let mut aggregater = AnyReportAggregater::new();
        for filter in &filter_set.filters {
            aggregater.join(self.check(filter, depth - 1)?);
        }
        aggregater.to_any()
    }

    fn filter_as_num(&self, num: usize, &range_operator: &RangeOperator) -> AnyReport {
        // TODO: Only report when `num` is on AS path.
        let routes = match self.compare.dump.as_routes.get(&num) {
            Some(r) => r,
            None => return skip_any_report(SkipReason::AsRoutesUnrecorded(num)),
        };
        let ranges: Vec<_> = routes
            .iter()
            .map(|&address_prefix| AddrPfxRange {
                address_prefix,
                range_operator,
            })
            .collect();
        let (reports, all_fail) = self.filter_prefixes(&ranges)?;
        if all_fail {
            no_match_any_report(format!(
                "{} does not match filter AS{num}{range_operator}",
                self.compare.prefix
            ))
        } else {
            Some((reports, all_fail))
        }
    }

    fn filter_prefixes<I>(&self, prefixes: I) -> AnyReport
    where
        I: IntoIterator<Item = &'a AddrPfxRange>,
    {
        prefixes
            .into_iter()
            .all(|prefix| !prefix.contains(&self.compare.prefix))
            .then(|| {
                let errors = vec![NoMatch(format!(
                    "{} does not match filter prefixes",
                    self.compare.prefix
                ))];
                (errors, true)
            })
    }

    fn filter_route_set(&self, name: &str, op: &RangeOperator, depth: isize) -> AnyReport {
        if depth <= 0 {
            return recursion_any_report(RecurSrc::FilterRouteSet(name.into()));
        }
        let route_set = match self.compare.dump.route_sets.get(name) {
            Some(r) => r,
            None => return skip_any_report(SkipReason::RouteSetUnrecorded(name.into())),
        };
        let mut aggregater = AnyReportAggregater::new();
        for member in &route_set.members {
            aggregater.join(self.filter_route_set_member(member, op, depth - 1)?);
        }
        if aggregater.all_fail {
            no_match_any_report(format!(
                "{} does no match filter route set {name}",
                self.compare.prefix
            ))
        } else {
            aggregater.to_any()
        }
    }

    fn filter_route_set_member(
        &self,
        member: &RouteSetMember,
        op: &RangeOperator,
        depth: isize,
    ) -> AnyReport {
        if depth <= 0 {
            return recursion_any_report(RecurSrc::FilterRouteSetMember(member.clone()));
        }
        match member {
            RouteSetMember::Range(prefix) => match (prefix.range_operator, op) {
                (RangeOperator::NoOp, RangeOperator::NoOp) => self.filter_prefixes([prefix]),
                (RangeOperator::NoOp, op) => self.filter_prefixes([&AddrPfxRange {
                    range_operator: *op,
                    ..prefix.clone()
                }]),
                _ => self.filter_prefixes([prefix]),
            },
            RouteSetMember::NameOp(name, op) => self.filter_route_set(name, op, depth - 1),
        }
    }

    fn filter_as_set<'v>(
        &self,
        name: &'a str,
        op: &RangeOperator,
        depth: isize,
        visited: &'v mut Vec<&'a AsName>,
    ) -> AnyReport {
        if depth <= 0 {
            return recursion_any_report(RecurSrc::FilterAsSet(name.into()));
        }
        let as_set = match self.compare.dump.as_sets.get(name) {
            Some(r) => r,
            None => return skip_any_report(SkipReason::AsSetUnrecorded(name.into())),
        };
        let mut aggregater = AnyReportAggregater::new();
        for as_name in &as_set.members {
            aggregater.join(self.filter_as_name(as_name, op, depth - 1, visited)?);
        }
        aggregater.to_any()
    }

    fn filter_as_regex(&self, expr: &str) -> AnyReport {
        // TODO: Implement.
        skip_any_report(SkipReason::AsRegexUnimplemented(expr.into()))
    }

    fn filter_as_name<'v>(
        &self,
        as_name: &'a AsName,
        op: &RangeOperator,
        depth: isize,
        visited: &'v mut Vec<&'a AsName>,
    ) -> AnyReport {
        if visited.iter().any(|x| **x == *as_name) {
            return no_match_any_report(format!("filter_as_name visited {as_name:?} before"));
        }
        visited.push(as_name);
        if depth <= 0 {
            return recursion_any_report(RecurSrc::FilterAsName(as_name.clone()));
        }
        match as_name {
            AsName::Num(num) => self.filter_as_num(*num, op),
            AsName::Set(name) => self.filter_as_set(name, op, depth - 1, visited),
            AsName::Invalid(reason) => {
                bad_rpsl_any_report(format!("Invalid AS name in filter: {reason}"))
            }
        }
    }

    fn filter_and(&self, left: &'a Filter, right: &'a Filter, depth: isize) -> AllReport {
        if depth <= 0 {
            return recursion_all_report(RecurSrc::FilterAnd);
        }
        self.check(left, depth - 1)
            .to_all()?
            .join(self.check(right, depth).to_all()?)
            .to_all()
    }

    fn filter_or(&self, left: &'a Filter, right: &'a Filter, depth: isize) -> AnyReport {
        if depth <= 0 {
            return recursion_any_report(RecurSrc::FilterOr);
        }
        let mut report: AnyReportAggregater = self.check(left, depth - 1)?.into();
        report.join(self.check(right, depth)?);
        report.to_any()
    }

    fn filter_not(&self, filter: &'a Filter, depth: isize) -> AnyReport {
        if depth <= 0 {
            return recursion_any_report(RecurSrc::FilterNot);
        }
        match self.check(filter, depth) {
            Some((_errors, true)) => None,
            Some((mut skips, false)) => {
                skips.push(Skip(SkipReason::SkippedNotFilterResult));
                Some((skips, false))
            }
            None => Some((
                vec![NoMatch(format!(
                    "AS{} from {} matches NOT filter {filter:?}",
                    self.accept_num, self.compare.prefix
                ))],
                true,
            )),
        }
    }

    fn filter_community(&self, community: &Call) -> AnyReport {
        // TODO: Implement.
        skip_any_report(SkipReason::CommunityCheckUnimplemented(community.clone()))
    }

    fn invalid_filter(&self, reason: &str) -> AnyReport {
        bad_rpsl_any_report(format!("Invalid filter: {reason}"))
    }
}
