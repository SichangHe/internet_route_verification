use crate::parse::{
    aut_sys::AsName,
    peering::{AsExpr, Peering},
    router_expr::RouterExpr,
};

use super::{
    cmp::{Compare, RECURSION_ERROR},
    report::{
        bad_rpsl_any_report, no_match_all_report, no_match_any_report, skip_any_report, AllReport,
        AnyReport, AnyReportAggregater, JoinReportItems, ReportItem::*, ToAllReport, ToAnyReport,
    },
};

pub struct CheckPeering<'a> {
    pub compare: &'a Compare<'a>,
    pub accept_num: usize,
}

impl<'a> CheckPeering<'a> {
    pub fn check(
        &self,
        Peering {
            remote_as,
            remote_router,
            local_router,
        }: &Peering,
        depth: isize,
    ) -> AllReport {
        self.check_remote_route(remote_router.as_ref())
            .to_all()?
            .join(self.check_local_route(local_router.as_ref()).to_all()?)
            .join(self.check_remote_as(remote_as, depth).to_all()?)
            .to_all()
    }

    fn check_remote_route(&self, remote_router: Option<&RouterExpr>) -> AnyReport {
        let _remote_router = remote_router?;
        // TODO: How to check this?
        None
    }

    fn check_local_route(&self, local_router: Option<&RouterExpr>) -> AnyReport {
        let _local_router = local_router?;
        // TODO: How to check this?
        None
    }

    fn check_remote_as(&self, remote_as: &AsExpr, depth: isize) -> AnyReport {
        if depth <= 0 {
            return no_match_any_report(format!("check_remote_as: {RECURSION_ERROR}"));
        }
        match remote_as {
            AsExpr::Single(as_name) => self.check_remote_as_name(as_name, depth),
            AsExpr::PeeringSet(name) => self.check_remote_peering_set(name, depth),
            AsExpr::And { left, right } => self.check_and(left, right, depth).to_any(),
            AsExpr::Or { left, right } => self.check_or(left, right, depth),
            AsExpr::Except { left, right } => self.check_except(left, right, depth).to_any(),
            AsExpr::Group(remote_as) => self.check_remote_as(remote_as, depth),
        }
    }

    fn check_remote_as_name(&self, as_name: &AsName, depth: isize) -> AnyReport {
        if depth <= 0 {
            return no_match_any_report(format!("check_remote_as_name: {RECURSION_ERROR}"));
        }
        match as_name {
            AsName::Num(num) => self.check_remote_as_num(*num),
            AsName::Set(name) => self.check_remote_as_set(name, depth),
            AsName::Invalid(reason) => {
                bad_rpsl_any_report(format!("{reason} when checking peering"))
            }
        }
    }

    fn check_remote_as_num(&self, num: usize) -> AnyReport {
        if self.accept_num == num {
            None
        } else {
            no_match_any_report(format!(
                "AS{} does not match peering {num} in remote AS name",
                self.accept_num
            ))
        }
    }

    fn check_remote_as_set(&self, name: &str, depth: isize) -> AnyReport {
        if depth <= 0 {
            return no_match_any_report(format!("check_remote_as_set: {RECURSION_ERROR}"));
        }
        let as_set = match self.compare.dump.as_sets.get(name) {
            Some(r) => r,
            None => return skip_any_report(format!("{name} is not a recorded AS Set")),
        };
        let mut aggregater = AnyReportAggregater::new();
        for as_name in &as_set.members {
            aggregater.join(self.check_remote_as_name(as_name, depth - 1)?);
        }
        aggregater.to_any()
    }

    fn check_remote_peering_set(&self, name: &str, depth: isize) -> AnyReport {
        if depth <= 0 {
            return no_match_any_report(format!("check_remote_peering_set: {RECURSION_ERROR}"));
        }
        let peering_set = match self.compare.dump.peering_sets.get(name) {
            Some(r) => r,
            None => return skip_any_report(format!("{name} is not a recorded Peering Set")),
        };
        let mut aggregater = AnyReportAggregater::new();
        for peering in &peering_set.peerings {
            aggregater.join(self.check(peering, depth - 1).to_any()?);
        }
        aggregater.to_any()
    }

    fn check_and(&self, left: &AsExpr, right: &AsExpr, depth: isize) -> AllReport {
        if depth <= 0 {
            return no_match_all_report(format!("check_and: {RECURSION_ERROR}"));
        }
        self.check_remote_as(left, depth - 1)
            .to_all()?
            .join(self.check_remote_as(right, depth).to_all()?)
            .to_all()
    }

    fn check_or(&self, left: &AsExpr, right: &AsExpr, depth: isize) -> AnyReport {
        if depth <= 0 {
            return no_match_any_report(format!("check_or: {RECURSION_ERROR}"));
        }
        let mut report: AnyReportAggregater = self.check_remote_as(left, depth - 1)?.into();
        report.join(self.check_remote_as(right, depth)?);
        report.to_any()
    }

    fn check_except(&self, left: &AsExpr, right: &AsExpr, depth: isize) -> AllReport {
        if depth <= 0 {
            return no_match_all_report(format!("check_except: {RECURSION_ERROR}"));
        }
        let left_report = self.check_remote_as(left, depth - 1).to_all()?;
        let right_report = match self.check_remote_as(right, depth) {
            Some((_, true)) => Ok(None),
            Some((mut skips, false)) => {
                skips.push(Skip(format!(
                    "Skipping EXCEPT peering {right:?} due to skipped results"
                )));
                Ok(Some(skips))
            }
            None => no_match_all_report(format!(
                "AS{} matches right of EXCEPT peering {right:?}",
                self.accept_num
            )),
        };
        left_report.join(right_report?).to_all()
    }
}
