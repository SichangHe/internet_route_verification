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
    cmp::{Compare, RECURSION_ERROR, RECURSION_LIMIT},
    report::{
        bad_rpsl_any_report, no_match_all_report, no_match_any_report, skip_any_report, AllReport,
        AnyReport, AnyReportAggregater, JoinReportItems, ReportItem::*, ToAllReport, ToAnyReport,
    },
};

pub struct CheckFilter<'a> {
    pub compare: &'a Compare<'a>,
    pub accept_num: usize,
    pub call_depth: usize,
}

impl<'a> CheckFilter<'a> {
    pub fn check(&mut self, filter: &'a Filter) -> AnyReport {
        match filter {
            FilterSetName(name) => self.filter_set_name(name),
            Any => None,
            AddrPrefixSet(prefixes) => self.filter_prefixes(prefixes),
            RouteSet(name, op) => self.filter_route_set(name, op),
            AsNum(num, op) => self.filter_as_num(*num, op),
            AsSet(name, op) => self.filter_as_set(name, op, &mut Vec::new()),
            AsPathRE(expr) => self.filter_as_regex(expr),
            And { left, right } => self.filter_and(left, right).to_any(),
            Or { left, right } => self.filter_or(left, right),
            Not(filter) => self.filter_not(filter),
            Group(filter) => self.check(filter),
            Community(community) => self.filter_community(community),
            Illegal(reason) => self.illegal_filter(reason),
        }
    }

    fn check_recursion(&mut self) -> bool {
        self.call_depth += 1;
        self.call_depth >= RECURSION_LIMIT
    }

    fn filter_set_name(&self, name: &str) -> AnyReport {
        // TODO: Implement.
        skip_any_report(format!("Filter set {name} check is not implemented"))
    }

    fn filter_as_num(&self, num: usize, _op: &RangeOperator) -> AnyReport {
        // TODO: Implement.
        skip_any_report(format!("AS number {num} check is not implemented"))
        // TODO: Below is incorrect.
        // (num != self.accept_num).then(|| {
        //     let errors = vec![NoMatch(format!(
        //         "AS{} does not match {num}",
        //         self.accept_num
        //     ))];
        //     (errors, true)
        // })
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

    fn filter_route_set(&mut self, name: &str, op: &RangeOperator) -> AnyReport {
        if self.check_recursion() {
            return no_match_any_report(format!(
                "filter_route_set: {RECURSION_ERROR} checking {name}"
            ));
        }
        let route_set = match self.compare.dump.route_sets.get(name) {
            Some(r) => r,
            None => return skip_any_report(format!("{name} is not a recorded Route Set")),
        };
        let mut aggregater = AnyReportAggregater::new();
        for member in &route_set.members {
            aggregater.join(self.filter_route_set_member(member, op)?);
        }
        aggregater.to_any()
    }

    fn filter_route_set_member(
        &mut self,
        member: &RouteSetMember,
        op: &RangeOperator,
    ) -> AnyReport {
        if self.check_recursion() {
            return no_match_any_report(format!("filter_route_set_member: {RECURSION_ERROR}"));
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
            RouteSetMember::NameOp(name, op) => self.filter_route_set(name, op),
        }
    }

    fn filter_as_set<'v>(
        &mut self,
        name: &'a str,
        op: &RangeOperator,
        visited: &'v mut Vec<&'a AsName>,
    ) -> AnyReport {
        if self.check_recursion() {
            return no_match_any_report(format!(
                "filter_as_set: {RECURSION_ERROR} checking {name}"
            ));
        }
        let as_set = match self.compare.dump.as_sets.get(name) {
            Some(r) => r,
            None => return skip_any_report(format!("{name} is not a recorded AS Set")),
        };
        let mut aggregater = AnyReportAggregater::new();
        for as_name in &as_set.members {
            let (report, abort) = self.filter_as_name(as_name, op, visited);
            aggregater.join(report?);
            if abort {
                break;
            }
        }
        aggregater.to_any()
    }

    fn filter_as_regex(&self, expr: &str) -> AnyReport {
        // TODO: Implement.
        skip_any_report(format!("AS regex {expr} check is not implemented"))
    }

    fn filter_as_name<'v>(
        &mut self,
        as_name: &'a AsName,
        op: &RangeOperator,
        visited: &'v mut Vec<&'a AsName>,
    ) -> (AnyReport, bool) {
        if visited.iter().any(|x| **x == *as_name) {
            let report = bad_rpsl_any_report(format!("filter_as_name visited {as_name:?} before"));
            return (report, true);
        }
        visited.push(as_name);
        if self.check_recursion() {
            let report = no_match_any_report(format!(
                "filter_as_name: {RECURSION_ERROR} checking {as_name:?}"
            ));
            return (report, true);
        }
        let report = match as_name {
            AsName::Num(num) => self.filter_as_num(*num, op),
            AsName::Set(name) => self.filter_as_set(name, op, visited),
            AsName::Illegal(reason) => {
                bad_rpsl_any_report(format!("Illegal AS name in filter: {reason}"))
            }
        };
        (report, false)
    }

    fn filter_and(&mut self, left: &'a Filter, right: &'a Filter) -> AllReport {
        if self.check_recursion() {
            return no_match_all_report(format!("filter_and: {RECURSION_ERROR}"));
        }
        self.check(left)
            .to_all()?
            .join(self.check(right).to_all()?)
            .to_all()
    }

    fn filter_or(&mut self, left: &'a Filter, right: &'a Filter) -> AnyReport {
        if self.check_recursion() {
            return no_match_any_report(format!("filter_or: {RECURSION_ERROR}"));
        }
        let mut report: AnyReportAggregater = self.check(left)?.into();
        report.join(self.check(right)?);
        report.to_any()
    }

    fn filter_not(&mut self, filter: &'a Filter) -> AnyReport {
        if self.check_recursion() {
            return no_match_any_report(format!("filter_not: {RECURSION_ERROR}"));
        }
        match self.check(filter) {
            Some((_errors, true)) => None,
            Some((mut skips, false)) => {
                skips.push(Skip(format!(
                    "Skipping NOT filter {filter:?} due to skipped results"
                )));
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
        skip_any_report(format!("Community {community:?} check is not implemented"))
    }

    fn illegal_filter(&self, reason: &str) -> AnyReport {
        bad_rpsl_any_report(format!("Illegal filter: {reason}"))
    }
}
