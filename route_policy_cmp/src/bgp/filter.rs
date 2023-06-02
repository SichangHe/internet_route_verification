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
    report::{
        bad_rpsl_any_report, skip_any_report, AllReport, AnyReport, AnyReportAggregater,
        JoinReportItems, ReportItem::*, ToAllReport, ToAnyReport,
    },
};

pub struct CheckFilter<'a> {
    pub compare: &'a Compare<'a>,
    pub accept_num: usize,
}

impl<'a> CheckFilter<'a> {
    pub fn check(&self, filter: &Filter) -> AnyReport {
        match filter {
            FilterSetName(name) => self.filter_set_name(name),
            Any => None,
            AddrPrefixSet(prefixes) => self.filter_prefixes(prefixes),
            RouteSet(name, op) => self.filter_route_set(name, op),
            AsNum(num, op) => self.filter_as_num(*num, op),
            AsSet(name, op) => self.filter_as_set(name, op),
            AsPathRE(expr) => self.filter_as_regex(expr),
            And { left, right } => self.filter_and(left, right).to_any(),
            Or { left, right } => self.filter_or(left, right),
            Not(filter) => self.filter_not(filter),
            Group(filter) => self.check(filter),
            Community(community) => self.filter_community(community),
            Illegal(reason) => self.illegal_filter(reason),
        }
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

    fn filter_route_set(&self, name: &str, op: &RangeOperator) -> AnyReport {
        let route_set = match self.compare.dump.route_sets.get(name) {
            Some(r) => r,
            None => {
                let errors = vec![Skip(format!("{name} is not a recorded Route Set"))];
                return Some((errors, false));
            }
        };
        let mut aggregater = AnyReportAggregater::new();
        for member in &route_set.members {
            aggregater.join(self.filter_route_set_member(member, op)?);
        }
        aggregater.to_some()
    }

    fn filter_route_set_member(&self, member: &RouteSetMember, op: &RangeOperator) -> AnyReport {
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

    fn filter_as_set(&self, name: &str, op: &RangeOperator) -> AnyReport {
        let as_set = match self.compare.dump.as_sets.get(name) {
            Some(r) => r,
            None => {
                let errors = vec![Skip(format!("{name} is not a recorded AS Set"))];
                return Some((errors, true));
            }
        };
        let mut aggregater = AnyReportAggregater::new();
        for as_name in &as_set.members {
            aggregater.join(self.filter_as_name(as_name, op)?);
        }
        aggregater.to_some()
    }

    fn filter_as_regex(&self, expr: &str) -> AnyReport {
        // TODO: Implement.
        skip_any_report(format!("AS regex {expr} check is not implemented"))
    }

    fn filter_as_name(&self, as_name: &AsName, op: &RangeOperator) -> AnyReport {
        match as_name {
            AsName::Num(num) => self.filter_as_num(*num, op),
            AsName::Set(name) => self.filter_as_set(name, op),
            AsName::Illegal(reason) => {
                bad_rpsl_any_report(format!("Illegal AS name in filter: {reason}"))
            }
        }
    }

    fn filter_and(&self, left: &Filter, right: &Filter) -> AllReport {
        // Assume `left` cannot be "And" or "Or".
        let report = self.check(left).to_all()?;
        match right {
            And { left, right } => Ok(report.join(self.filter_and(left, right)?)),
            Or { left, right } => Ok(report.join(self.filter_or(left, right).to_all()?)),
            right => Ok(report.join(self.check(right).to_all()?)),
        }
    }

    fn filter_or(&self, left: &Filter, right: &Filter) -> AnyReport {
        // Assume `left` cannot be "And" or "Or".
        let mut aggregater: AnyReportAggregater = self.check(left)?.into();
        match right {
            And { left, right } => aggregater.join(self.filter_and(left, right).to_any()?),
            Or { left, right } => aggregater.join(self.filter_or(left, right)?),
            right => aggregater.join(self.check(right)?),
        }
        aggregater.to_some()
    }

    fn filter_not(&self, filter: &Filter) -> AnyReport {
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
