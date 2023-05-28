use crate::parse::{
    address_prefix::AddrPfxRange,
    filter::Filter::{self, *},
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
            RouteSetName(_) => todo!(),
            AsNum(num, _) => (*num == self.accept_num).then_some(Good), // TODO: what about the operator?
            AsSet(_, _) => todo!(),
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

    fn filter_prefixes(&self, prefixes: &[AddrPfxRange]) -> Option<Report> {
        prefixes
            .iter()
            .any(|prefix| prefix.contains(&self.compare.prefix))
            .then_some(Good)
    }

    fn filter_and(&self, left_report: Report, right: &Filter) -> Option<Report> {
        // let left_report = self.check(left)?;
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
