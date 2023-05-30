use crate::parse::{
    address_prefix::AddrPfxRange,
    aut_sys::AsName,
    filter::{
        Filter::{self, *},
        RegexOperator,
    },
    set::RouteSetMember,
};

use super::{
    cmp::Compare,
    report::{
        self,
        Report::{self, *},
    },
};

pub struct CheckFilter<'a> {
    pub compare: &'a Compare<'a>,
    pub accept_num: usize,
}

impl<'a> CheckFilter<'a> {
    pub fn check(&self, filter: &Filter) -> Option<Report> {
        match filter {
            FilterSetName(_) => todo!(),
            Any => Some(Good),
            AddrPrefixSet(prefixes) => self.filter_prefixes(prefixes),
            RouteSetName(name) => self.filter_route_set_name(name),
            AsNum(num, _) => (*num == self.accept_num).then_some(Good), // TODO: what about the operator?
            AsSet(name, op) => self.filter_as_set_name(name, op),
            AsPathRE(_) => todo!(),
            PeerAs => todo!(),
            And { left, right } => self.filter_and(self.check(left)?, right),
            Or { left, right } => self.filter_or(left, right, None),
            Not(filter) => match self.check(filter) {
                report @ Some(Skip(_)) => report,
                Some(_) => None,
                None => Some(Good),
            },
            Group(filter) => self.check(filter),
            Community(_) => todo!(),
        }
    }

    fn filter_prefixes<I>(&self, prefixes: I) -> Option<Report>
    where
        I: IntoIterator<Item = &'a AddrPfxRange>,
    {
        prefixes
            .into_iter()
            .any(|prefix| prefix.contains(&self.compare.prefix))
            .then_some(Good)
    }

    fn filter_route_set_name(&self, name: &str) -> Option<Report> {
        let route_set = match self.compare.dump.route_sets.get(name) {
            Some(r) => r,
            None => return Some(Skip(format!("{name} is not a recorded Route Set"))),
        };
        let mut report = None;
        for member in &route_set.members {
            match self.filter_route_set_member(member) {
                Some(Good) => return Some(Good),
                new_report => report = report::or(report, new_report),
            }
        }
        report
    }

    fn filter_route_set_member(&self, member: &RouteSetMember) -> Option<Report> {
        match member {
            RouteSetMember::Range(prefix) => self.filter_prefixes([prefix]),
            RouteSetMember::Name(name) => self.filter_route_set_name(name),
            RouteSetMember::NameOp(_, _) => todo!(),
        }
    }

    fn filter_as_set_name(&self, name: &str, op: &RegexOperator) -> Option<Report> {
        let as_set = match self.compare.dump.as_sets.get(name) {
            Some(r) => r,
            None => return Some(Skip(format!("{name} is not a recorded AS Set"))),
        };
        let mut report = None;
        for as_name in &as_set.members {
            match self.filter_as_name(as_name, op) {
                Some(Good) => return Some(Good),
                new_report => report = report::or(report, new_report),
            }
        }
        report
    }

    fn filter_as_name(&self, as_name: &AsName, op: &RegexOperator) -> Option<Report> {
        match as_name {
            AsName::Num(_) => todo!(),
            AsName::Set(_) => todo!(),
            AsName::Illegal(_) => todo!(),
        }
    }

    fn filter_and(&self, left_report: Report, right: &Filter) -> Option<Report> {
        match right {
            And { left, right } => self.filter_and(left_report & self.check(left)?, right),
            Or { left, right } => self.filter_or(left, right, Some(left_report)),
            right => Some(left_report & self.check(right)?),
        }
    }

    fn filter_or(&self, left: &Filter, right: &Filter, report: Option<Report>) -> Option<Report> {
        let left_report = self.check(left);
        if let Some(Good) = left_report {
            return Some(Good);
        }
        let report = report::or(report, left_report);
        match right {
            And { left, right } => {
                self.filter_and(report::or_known(report, self.check(left)?), right)
            }
            Or { left, right } => self.filter_or(left, right, report),
            right => report::or(report, self.check(right)),
        }
    }
}
