use parse::*;

use super::*;

use {OkTBad::*, SkipFBad::*};

pub struct CheckPeering<'a> {
    pub dump: &'a QueryDump,
    pub compare: &'a Compare,
    pub accept_num: u64,
    pub verbosity: Verbosity,
}

impl<'a> CheckPeering<'a> {
    /// Do not check `remote_router` or `local_router` because we do not have
    /// the router information needed.
    pub fn check(
        &self,
        Peering {
            remote_as,
            remote_router: _,
            local_router: _,
        }: &Peering,
        depth: isize,
    ) -> AllReport {
        self.check_remote_as(remote_as, depth).to_all()
    }

    fn check_remote_as(&self, remote_as: &AsExpr, depth: isize) -> AnyReport {
        if depth <= 0 {
            return recursion_any_report(RecurSrc::CheckRemoteAs);
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
            return recursion_any_report(RecurSrc::RemoteAsName(as_name.clone()));
        }
        match as_name {
            AsName::Num(num) => self.check_remote_as_num(*num),
            AsName::Set(name) => {
                self.check_remote_as_set(name, depth, &mut BloomHashSet::with_capacity(2048, 32768))
            }
            AsName::Invalid(reason) => {
                self.bad_rpsl_any_report(|| RpslError::InvalidAsName(reason.into()))
            }
        }
    }

    fn check_remote_as_num(&self, num: u64) -> AnyReport {
        if self.accept_num == num {
            None
        } else {
            self.no_match_any_report(|| MatchProblem::RemoteAsNum(num))
        }
    }

    fn check_remote_as_set(
        &self,
        name: &'a str,
        depth: isize,
        visited: &mut BloomHashSet<&'a str>,
    ) -> AnyReport {
        let hash = visited.make_hash(&name);
        if visited.contains_with_hash(&name, hash) {
            return failed_any_report();
        }

        if depth <= 0 {
            return recursion_any_report(RecurSrc::RemoteAsSet(name.into()));
        }
        let as_set = match self.dump.as_sets.get(name) {
            Some(r) => r,
            None => return self.skip_any_report(|| SkipReason::AsSetUnrecorded(name.into())),
        };

        if as_set.members.binary_search(&self.accept_num).is_ok() {
            return None;
        }

        self.check_remote_as_set_members(name, depth, visited, hash, as_set)
    }

    fn check_remote_as_set_members(
        &self,
        name: &'a str,
        depth: isize,
        visited: &mut BloomHashSet<&'a str>,
        hash: u64,
        as_set: &'a AsSet,
    ) -> AnyReport {
        visited.insert_with_hash(name, hash);

        let mut report = SkipFBad::const_default();
        for set in &as_set.set_members {
            report |= self.check_remote_as_set(set, depth - 1, visited)?;
        }
        if let BadF(_) = report {
            self.no_match_any_report(|| MatchProblem::RemoteAsSet(name.into()))
        } else {
            Some(report)
        }
    }
    fn check_remote_peering_set(&self, name: &str, depth: isize) -> AnyReport {
        if depth <= 0 {
            return recursion_any_report(RecurSrc::RemotePeeringSet(name.into()));
        }
        let peering_set = match self.dump.peering_sets.get(name) {
            Some(r) => r,
            None => return self.skip_any_report(|| SkipReason::PeeringSetUnrecorded(name.into())),
        };
        let mut report = SkipFBad::const_default();
        for peering in &peering_set.peerings {
            report |= self.check(peering, depth - 1).to_any()?;
        }
        Some(report)
    }

    fn check_and(&self, left: &AsExpr, right: &AsExpr, depth: isize) -> AllReport {
        if depth <= 0 {
            return recursion_all_report(RecurSrc::PeeringAnd);
        }
        Ok(self.check_remote_as(left, depth - 1).to_all()?
            & self.check_remote_as(right, depth).to_all()?)
    }

    fn check_or(&self, left: &AsExpr, right: &AsExpr, depth: isize) -> AnyReport {
        if depth <= 0 {
            return recursion_any_report(RecurSrc::PeeringOr);
        }
        Some(self.check_remote_as(left, depth - 1)? | self.check_remote_as(right, depth)?)
    }

    fn check_except(&self, left: &AsExpr, right: &AsExpr, depth: isize) -> AllReport {
        if depth <= 0 {
            return recursion_all_report(RecurSrc::PeeringExcept);
        }
        Ok(self.check_remote_as(left, depth - 1).to_all()?
            & match self.check_remote_as(right, depth) {
                report @ Some(SkipF(_)) | report @ Some(MehF(_)) => {
                    report.to_all()?
                        & self.skip_all_report(|| SkipReason::SkippedExceptPeeringResult)?
                }
                Some(BadF(_)) => OkT,
                None => self.no_match_all_report(|| MatchProblem::ExceptPeeringRightMatch)?,
            })
    }
}

impl<'a> VerbosityReport for CheckPeering<'a> {
    fn get_verbosity(&self) -> Verbosity {
        self.verbosity
    }
}
