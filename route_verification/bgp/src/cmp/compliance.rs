use super::*;

pub struct Compliance<'a> {
    pub cmp: &'a Compare,
    pub query: &'a QueryIr,
    pub accept_num: u32,
    pub self_num: u32,
    pub export: bool,
    pub prev_path: &'a [AsPathEntry],
}

impl<'a> Compliance<'a> {
    pub fn check_new(&self, policy: &Versions) -> AnyReport {
        Some(
            match self.cmp.prefix {
                IpNet::V4(_) => self.check_casts_new(&policy.ipv4),
                IpNet::V6(_) => self.check_casts_new(&policy.ipv6),
            }? | self.check_casts_new(&policy.any)?,
        )
    }

    pub fn check(&self, policy: &Versions) -> AnyReport {
        Some(
            match self.cmp.prefix {
                IpNet::V4(_) => self.check_casts(&policy.ipv4),
                IpNet::V6(_) => self.check_casts(&policy.ipv6),
            }? | self.check_casts(&policy.any)?,
        )
    }

    pub fn check_casts_new(&self, casts: &Casts) -> AnyReport {
        let mut report = AnyReportCase::const_default();
        let specific_cast = match is_multicast(&self.cmp.prefix) {
            true => &casts.multicast,
            false => &casts.unicast,
        };
        for entry in [specific_cast, &casts.any].into_iter().flatten() {
            report |= self.check_entry_new(entry).to_any()?;
        }
        Some(report)
    }

    pub fn check_casts(&self, casts: &Casts) -> AnyReport {
        let mut report = AnyReportCase::const_default();
        let specific_cast = match is_multicast(&self.cmp.prefix) {
            true => &casts.multicast,
            false => &casts.unicast,
        };
        for entry in [specific_cast, &casts.any].into_iter().flatten() {
            report |= self.check_entry(entry).to_any()?;
        }
        Some(report)
    }

    pub fn check_entry_new(&self, entry: &Entry) -> AllReport {
        let peering_report = CheckPeering {
            c: self,
            accept_num: self.accept_num,
        }
        .check_peering_actions(&entry.mp_peerings)
        .to_all()
        .map_err(|mut report| {
            if self.cmp.verbosity.per_peering_err {
                report.push(MatchPeering);
            }
            report
        })?;
        let filter_report = CheckFilter {
            cmp: self.cmp,
            query: self.query,
            self_num: self.self_num,
            export: self.export,
            prev_path: self.prev_path,
            mp_peerings: &entry.mp_peerings,
        }
        .check_filter_new(&entry.mp_filter, self.cmp.recursion_limit)
        .to_all()
        .map_err(|mut report| {
            if self.cmp.verbosity.per_filter_err {
                report.push(MatchFilter);
            }
            report
        })?;
        Ok(peering_report & filter_report)
    }

    pub fn check_entry(&self, entry: &Entry) -> AllReport {
        let peering_report = CheckPeering {
            c: self,
            accept_num: self.accept_num,
        }
        .check_peering_actions(&entry.mp_peerings)
        .to_all()
        .map_err(|mut report| {
            if self.cmp.verbosity.per_peering_err {
                report.push(MatchPeering);
            }
            report
        })?;
        let filter_report = CheckFilter {
            cmp: self.cmp,
            query: self.query,
            self_num: self.self_num,
            export: self.export,
            prev_path: self.prev_path,
            mp_peerings: &entry.mp_peerings,
        }
        .check_filter(&entry.mp_filter, self.cmp.recursion_limit)
        .to_all()
        .map_err(|mut report| {
            if self.cmp.verbosity.per_filter_err {
                report.push(MatchFilter);
            }
            report
        })?;
        Ok(peering_report & filter_report)
    }
}

impl<'a> VerbosityReport for Compliance<'a> {
    fn get_verbosity(&self) -> Verbosity {
        self.cmp.verbosity
    }
}
