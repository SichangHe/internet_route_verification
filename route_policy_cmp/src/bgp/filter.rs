use crate::bgp::report::AllReport;
use crate::parse::{
    address_prefix::AddrPfxRange,
    aut_sys::AsName,
    filter::{
        Filter::{self, *},
        RegexOperator,
    },
    set::RouteSetMember,
};

use super::report::{AnyReportAggregater, JoinReportItems, ToAllReport, ToAnyReport};
use super::{
    cmp::Compare,
    report::{
        AnyReport,
        ReportItem::{self, *},
    },
};

pub struct CheckFilter<'a> {
    pub compare: &'a Compare<'a>,
    pub accept_num: usize,
}

impl<'a> CheckFilter<'a> {
    pub fn check(&self, filter: &Filter) -> AnyReport {
        match filter {
            FilterSetName(_) => todo!(),
            Any => None,
            AddrPrefixSet(prefixes) => self.filter_prefixes(prefixes),
            RouteSetName(name) => self.filter_route_set_name(name),
            AsNum(num, _) => self.filter_as_num(*num),
            AsSet(name, op) => self.filter_as_set_name(name, op),
            AsPathRE(_) => todo!(),
            PeerAs => todo!(),
            And { left, right } => self.filter_and(left, right).to_any(),
            Or { left, right } => self.filter_or(left, right),
            Not(filter) => self.filter_not(filter),
            Group(filter) => self.check(filter),
            Community(_) => todo!(),
        }
    }

    fn filter_as_num(&self, num: usize) -> AnyReport {
        // TODO: what about the operator?
        (num != self.accept_num).then(|| {
            let errors = vec![NoMatch(format!(
                "AS{} does not match {num}",
                self.accept_num
            ))];
            (errors, true)
        })
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

    fn filter_route_set_name(&self, name: &str) -> AnyReport {
        let route_set = match self.compare.dump.route_sets.get(name) {
            Some(r) => r,
            None => {
                let errors = vec![Skip(format!("{name} is not a recorded Route Set"))];
                return Some((errors, false));
            }
        };
        let mut aggregater = AnyReportAggregater::new();
        for member in &route_set.members {
            aggregater.join(self.filter_route_set_member(member)?);
        }
        aggregater.to_some()
    }

    fn filter_route_set_member(&self, member: &RouteSetMember) -> AnyReport {
        match member {
            RouteSetMember::Range(prefix) => self.filter_prefixes([prefix]),
            RouteSetMember::Name(name) => self.filter_route_set_name(name),
            RouteSetMember::NameOp(_, _) => todo!(),
        }
    }

    fn filter_as_set_name(&self, name: &str, op: &RegexOperator) -> AnyReport {
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

    fn filter_as_name(&self, as_name: &AsName, _op: &RegexOperator) -> AnyReport {
        match as_name {
            AsName::Num(_) => todo!(),
            AsName::Set(_) => todo!(),
            AsName::Illegal(_) => todo!(),
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
                skips.push(ReportItem::Skip(format!(
                    "Skipping NOT filter {filter:?} due to skipped results"
                )));
                Some((skips, false))
            }
            None => Some((
                vec![ReportItem::NoMatch(format!(
                    "AS{} from {} matches NOT filter {filter:?}",
                    self.accept_num, self.compare.prefix
                ))],
                true,
            )),
        }
    }
}
