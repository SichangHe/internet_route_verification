use parse::*;

use super::*;

pub struct CheckPeering<'a> {
    pub c: &'a Compliance<'a>,
    pub accept_num: u64,
}

impl<'a> CheckPeering<'a> {
    pub fn check_peering_actions<'b, I>(&self, peerings: I) -> AnyReport
    where
        I: IntoIterator<Item = &'b PeeringAction>,
    {
        let mut report = AnyReportCase::const_default();
        for peering_actions in peerings.into_iter() {
            let new = self.check_peering_action(peering_actions);
            report |= new.to_any()?;
        }
        Some(report)
    }

    pub fn check_peering_action(&self, peering_actions: &PeeringAction) -> AllReport {
        self.check(&peering_actions.mp_peering, self.c.cmp.recursion_limit)
        // Skipped.
        /* ?
        .join(self.check_actions(&peering_actions.actions)?)
        .to_all()
        */
    }

    /// We skip community checks, but this could be an enhancement.
    /// <https://github.com/SichangHe/parse_rpsl_policy/issues/16>.
    pub fn check_actions(&self, _actions: &Actions) -> AllReport {
        Ok(OkAllReport)
    }

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
            return bad_any_report(RecCheckRemoteAs);
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
            return bad_any_report(RecRemoteAsName(Box::new(as_name.clone())));
        }
        match as_name {
            AsName::Any => None,
            AsName::Num(num) => self.check_remote_as_num(*num),
            AsName::Set(name) => {
                self.check_remote_as_set(name, depth, &mut BloomHashSet::with_capacity(2048, 32768))
            }
            AsName::Invalid(reason) => self.bad_any_report(|| RpslInvalidAsName(reason.into())),
        }
    }

    fn check_remote_as_num(&self, num: u64) -> AnyReport {
        if self.accept_num == num {
            None
        } else {
            self.bad_any_report(|| MatchRemoteAsNum(num))
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
            return empty_bad_any_report();
        }

        if depth <= 0 {
            return bad_any_report(RecRemoteAsSet(name.into()));
        }
        let as_set = match self.c.query.as_sets.get(name) {
            Some(r) => r,
            None => return self.skip_any_report(|| UnrecordedAsSet(name.into())),
        };

        if as_set.is_any || as_set.members.binary_search(&self.accept_num).is_ok() {
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

        let mut report = AnyReportCase::const_default();
        for set in &as_set.set_members {
            report |= self.check_remote_as_set(set, depth - 1, visited)?;
        }
        if let BadAnyReport(_) = report {
            self.bad_any_report(|| MatchRemoteAsSet(name.into()))
        } else {
            Some(report)
        }
    }
    fn check_remote_peering_set(&self, name: &str, depth: isize) -> AnyReport {
        if depth <= 0 {
            return bad_any_report(RecRemotePeeringSet(name.into()));
        }
        let peering_set = match self.c.query.peering_sets.get(name) {
            Some(r) => r,
            None => return self.skip_any_report(|| UnrecordedPeeringSet(name.into())),
        };
        let mut report = AnyReportCase::const_default();
        for peering in &peering_set.peerings {
            report |= self.check(peering, depth - 1).to_any()?;
        }
        Some(report)
    }

    fn check_and(&self, left: &AsExpr, right: &AsExpr, depth: isize) -> AllReport {
        if depth <= 0 {
            return bad_all_report(RecPeeringAnd);
        }
        Ok(self.check_remote_as(left, depth - 1).to_all()?
            & self.check_remote_as(right, depth).to_all()?)
    }

    fn check_or(&self, left: &AsExpr, right: &AsExpr, depth: isize) -> AnyReport {
        if depth <= 0 {
            return bad_any_report(RecPeeringOr);
        }
        Some(self.check_remote_as(left, depth - 1)? | self.check_remote_as(right, depth)?)
    }

    fn check_except(&self, left: &AsExpr, right: &AsExpr, depth: isize) -> AllReport {
        if depth <= 0 {
            return bad_all_report(RecPeeringExcept);
        }
        Ok(self.check_remote_as(left, depth - 1).to_all()?
            & match self.check_remote_as(right, depth) {
                report @ Some(SkipAnyReport(_)) | report @ Some(MehAnyReport(_)) => {
                    report.to_all()? & self.skip_all_report(|| SkipSkippedExceptPeeringResult)?
                }
                Some(BadAnyReport(_)) => OkAllReport,
                None => self.bad_all_report(|| MatchExceptPeeringRight)?,
            })
    }
}

impl<'a> VerbosityReport for CheckPeering<'a> {
    fn get_verbosity(&self) -> Verbosity {
        self.c.cmp.verbosity
    }
}
