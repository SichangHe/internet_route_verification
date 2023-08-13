use super::*;

#[derive(Clone)]
pub struct Compliance<'a> {
    pub cmp: &'a Compare,
    pub dump: &'a QueryDump,
    pub accept_num: Option<u64>,
    pub self_num: u64,
    pub export: bool,
    pub prev_path: &'a [AsPathEntry],
}
impl<'a> Compliance<'a> {
    pub fn check(&self, policy: &Versions) -> AnyReport {
        Some(
            match self.cmp.prefix {
                IpNet::V4(_) => self.check_casts(&policy.ipv4),
                IpNet::V6(_) => self.check_casts(&policy.ipv6),
            }? | self.check_casts(&policy.any)?,
        )
    }

    pub fn check_casts(&self, casts: &Casts) -> AnyReport {
        let mut report = SkipFBad::const_default();
        let specific_cast = match is_multicast(&self.cmp.prefix) {
            true => &casts.multicast,
            false => &casts.unicast,
        };
        for entry in [specific_cast, &casts.any].into_iter().flatten() {
            report |= self.check_entry(entry).to_any()?;
        }
        Some(report)
    }

    pub fn check_entry(&self, entry: &Entry) -> AllReport {
        let peering_report = match self.accept_num {
            Some(accept_num) => CheckPeering {
                c: self,
                accept_num,
            }
            .check_peering_actions(&entry.mp_peerings)
            .to_all()
            .map_err(|mut report| {
                if self.cmp.verbosity.per_entry_err {
                    report.push(NoMatch(Peering));
                }
                report
            })?,
            None => OkT,
        };
        let filter_report = self
            .check_filter(&entry.mp_filter, self.cmp.recursion_limit)
            .to_all()
            .map_err(|mut report| {
                if self.cmp.verbosity.per_entry_err {
                    report.push(NoMatch(Filter));
                }
                report
            })?;
        Ok(peering_report & filter_report)
    }

    /// `Err` contains all the skips.
    pub fn set_has_member(
        &self,
        set: &str,
        asn: u64,
        recursion_limit: isize,
    ) -> Result<bool, AnyReport> {
        if recursion_limit < 0 {
            return Err(recursion_any_report(RecurSrc::CheckSetMember(set.into())));
        }
        let set = match self.dump.as_sets.get(set) {
            Some(s) => s,
            None => return Err(self.skip_any_report(|| SkipReason::AsSetUnrecorded(set.into()))),
        };
        if set.members.contains(&asn) {
            return Ok(true);
        }
        let mut report = SkipF(vec![]);
        for set in &set.set_members {
            match self.set_has_member(set, asn, recursion_limit - 1) {
                Ok(true) => return Ok(true),
                Ok(false) => (),
                Err(err) => report |= err.unwrap(),
            }
        }
        match report {
            SkipF(items) if items.is_empty() => Ok(false),
            report => Err(Some(report)),
        }
    }
}

impl<'a> VerbosityReport for Compliance<'a> {
    fn get_verbosity(&self) -> Verbosity {
        self.cmp.verbosity
    }
}
