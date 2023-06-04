use crate::parse::{
    aut_sys::AsName,
    peering::{AsExpr, Peering},
    router_expr::RouterExpr,
};

use super::{
    cmp::{Compare, RECURSION_ERROR, RECURSION_LIMIT},
    report::{
        bad_rpsl_any_report, no_match_all_report, no_match_any_report, skip_any_report, AllReport,
        AnyReport, AnyReportAggregater, JoinReportItems, ReportItem::*, ToAllReport, ToAnyReport,
    },
};

pub struct CheckPeering<'a> {
    pub compare: &'a Compare<'a>,
    pub accept_num: usize,
    pub call_depth: usize,
}

impl<'a> CheckPeering<'a> {
    pub fn check(
        &mut self,
        Peering {
            remote_as,
            remote_router,
            local_router,
        }: &Peering,
    ) -> AllReport {
        self.check_remote_route(remote_router.as_ref())
            .to_all()?
            .join(self.check_local_route(local_router.as_ref()).to_all()?)
            .join(self.check_remote_as(remote_as).to_all()?)
            .to_all()
    }

    fn check_recursion(&mut self) -> bool {
        self.call_depth += 1;
        self.call_depth >= RECURSION_LIMIT
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

    fn check_remote_as(&mut self, remote_as: &AsExpr) -> AnyReport {
        match remote_as {
            AsExpr::Single(as_name) => self.check_remote_as_name(as_name),
            AsExpr::PeeringSet(name) => self.check_remote_peering_set(name),
            AsExpr::And { left, right } => self.check_and(left, right).to_any(),
            AsExpr::Or { left, right } => self.check_or(left, right),
            AsExpr::Except { left, right } => self.check_except(left, right).to_any(),
            AsExpr::Group(remote_as) => self.check_remote_as(remote_as),
        }
    }

    fn check_remote_as_name(&mut self, as_name: &AsName) -> AnyReport {
        if self.check_recursion() {
            return no_match_any_report(format!("check_remote_as_name: {RECURSION_ERROR}"));
        }
        match as_name {
            AsName::Num(num) => self.check_remote_as_num(*num),
            AsName::Set(name) => self.check_remote_as_set(name),
            AsName::Illegal(reason) => {
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

    fn check_remote_as_set(&mut self, name: &str) -> AnyReport {
        if self.check_recursion() {
            return no_match_any_report(format!("check_remote_as_set: {RECURSION_ERROR}"));
        }
        let as_set = match self.compare.dump.as_sets.get(name) {
            Some(r) => r,
            None => return skip_any_report(format!("{name} is not a recorded AS Set")),
        };
        let mut aggregater = AnyReportAggregater::new();
        for as_name in &as_set.members {
            aggregater.join(self.check_remote_as_name(as_name)?);
        }
        aggregater.to_any()
    }

    fn check_remote_peering_set(&mut self, name: &str) -> AnyReport {
        if self.check_recursion() {
            return no_match_any_report(format!("check_remote_peering_set: {RECURSION_ERROR}"));
        }
        let peering_set = match self.compare.dump.peering_sets.get(name) {
            Some(r) => r,
            None => return skip_any_report(format!("{name} is not a recorded Peering Set")),
        };
        let mut aggregater = AnyReportAggregater::new();
        for peering in &peering_set.peerings {
            aggregater.join(self.check(peering).to_any()?);
        }
        aggregater.to_any()
    }

    fn check_and(&mut self, left: &AsExpr, right: &AsExpr) -> AllReport {
        if self.check_recursion() {
            return no_match_all_report(format!("check_and: {RECURSION_ERROR}"));
        }
        self.check_remote_as(left)
            .to_all()?
            .join(self.check_remote_as(right).to_all()?)
            .to_all()
    }

    fn check_or(&mut self, left: &AsExpr, right: &AsExpr) -> AnyReport {
        if self.check_recursion() {
            return no_match_any_report(format!("check_or: {RECURSION_ERROR}"));
        }
        let mut report: AnyReportAggregater = self.check_remote_as(left)?.into();
        report.join(self.check_remote_as(right)?);
        report.to_any()
    }

    fn check_except(&mut self, left: &AsExpr, right: &AsExpr) -> AllReport {
        if self.check_recursion() {
            return no_match_all_report(format!("check_except: {RECURSION_ERROR}"));
        }
        let left_report = self.check_remote_as(left).to_all()?;
        let right_report = match self.check_remote_as(right) {
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
