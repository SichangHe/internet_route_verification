use std::net::{Ipv4Addr, Ipv6Addr};

use anyhow::Result;
use ipnet::{IpNet, Ipv4Net, Ipv6Net, PrefixLenError};

use crate::parse::{
    action::Actions,
    aut_num::AutNum,
    dump::Dump,
    mp_import::{Casts, Entry, Versions},
    peering::PeeringAction,
};

use super::{
    filter::CheckFilter,
    map::{parse_table_dump, AsPathEntry},
    peering::CheckPeering,
    report::*,
    verbosity::{Verbosity, VerbosityReport},
};

pub const RECURSION_LIMIT: isize = 0x100;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Compare {
    pub prefix: IpNet,
    pub as_path: Vec<AsPathEntry>,
    pub communities: Vec<String>,
    pub recursion_limit: isize,
    pub verbosity: Verbosity,
}

impl Compare {
    pub fn new(prefix: IpNet, as_path: Vec<AsPathEntry>, communities: Vec<String>) -> Self {
        Self {
            prefix,
            as_path,
            communities,
            recursion_limit: RECURSION_LIMIT,
            verbosity: Verbosity::ErrOnly,
        }
    }

    pub fn verbosity(self, verbosity: Verbosity) -> Self {
        Self { verbosity, ..self }
    }

    pub fn with_line_dump(line: &str) -> Result<Self> {
        let (prefix, as_path, _, communities) = parse_table_dump(line)?;
        let communities = communities.into_iter().map(ToOwned::to_owned).collect();
        Ok(Self::new(prefix, as_path, communities))
    }

    pub fn check(&self, dump: &Dump) -> Vec<Report> {
        let mut reports = Vec::with_capacity(self.as_path.len() * 2);
        reports.extend(self.check_last_export(dump));

        // Iterate the pairs in `as_path` from right to left, with overlaps.
        let pairs = self
            .as_path
            .iter()
            .rev()
            .zip(self.as_path.iter().rev().skip(1));
        let pair_reports = pairs.flat_map(|(from, to)| {
            if let (AsPathEntry::Seq(from), AsPathEntry::Seq(to)) = (from, to) {
                self.check_pair(dump, *from, *to)
            } else {
                vec![Report::skip(SkipReason::AsPathPairWithSet(
                    from.clone(),
                    to.clone(),
                ))]
            }
        });
        reports.extend(pair_reports);
        reports.shrink_to_fit();
        reports
    }

    pub fn check_last_export(&self, dump: &Dump) -> Option<Report> {
        match self.as_path.last()? {
            AsPathEntry::Seq(from) => self
                .get_aut_num_then(dump, *from, |from_an| {
                    self.check_export(dump, from_an, *from, None)
                })
                .or_else(|| self.success_report(|| SuccessType::ExportSingle(*from))),
            entry => self.skip_report(|| SkipReason::AsPathWithSet(entry.clone())),
        }
    }

    pub fn check_pair(&self, dump: &Dump, from: usize, to: usize) -> Vec<Report> {
        let from_report = self
            .get_aut_num_then(dump, from, |from_an| {
                self.check_export(dump, from_an, from, Some(to))
            })
            .or_else(|| self.success_report(|| SuccessType::Export(from, to)));
        let to_report = self
            .get_aut_num_then(dump, to, |to_an| self.check_import(dump, to_an, from, to))
            .or_else(|| self.success_report(|| SuccessType::Import(to, from)));
        [from_report, to_report].into_iter().flatten().collect()
    }

    pub fn get_aut_num_then<F>(&self, dump: &Dump, aut_num: usize, call: F) -> Option<Report>
    where
        F: Fn(&AutNum) -> Option<Report>,
    {
        match dump.aut_nums.get(&aut_num) {
            Some(aut_num) => call(aut_num),
            None => self.skip_report(|| SkipReason::AutNumUnrecorded(aut_num)),
        }
    }

    pub fn check_export(
        &self,
        dump: &Dump,
        from_an: &AutNum,
        from: usize,
        to: Option<usize>,
    ) -> Option<Report> {
        let mut aggregator = self.check_compliant(dump, &from_an.exports, to)?;
        if aggregator.all_fail {
            let reason = match to {
                Some(to) => MatchProblem::NoExportRule(from, to),
                None => MatchProblem::NoExportRuleSingle(from),
            };
            aggregator.join(no_match_any_report(reason).unwrap());
            Some(Report::Bad(aggregator.report_items))
        } else {
            self.skips_report(aggregator.report_items)
        }
    }

    pub fn check_import(
        &self,
        dump: &Dump,
        to_an: &AutNum,
        from: usize,
        to: usize,
    ) -> Option<Report> {
        let mut aggregator = self.check_compliant(dump, &to_an.imports, Some(from))?;
        if aggregator.all_fail {
            aggregator.join(no_match_any_report(MatchProblem::NoImportRule(to, from)).unwrap());
            Some(Report::Bad(aggregator.report_items))
        } else {
            self.skips_report(aggregator.report_items)
        }
    }

    pub fn check_compliant(
        &self,
        dump: &Dump,
        policy: &Versions,
        accept_num: Option<usize>,
    ) -> Option<AnyReportAggregator> {
        let mut aggregator: AnyReportAggregator = match self.prefix {
            IpNet::V4(_) => self.check_casts(dump, &policy.ipv4, accept_num),
            IpNet::V6(_) => self.check_casts(dump, &policy.ipv6, accept_num),
        }?
        .into();
        aggregator.join(self.check_casts(dump, &policy.any, accept_num)?);
        Some(aggregator)
    }

    pub fn check_casts(&self, dump: &Dump, casts: &Casts, accept_num: Option<usize>) -> AnyReport {
        let mut aggregator = AnyReportAggregator::new();
        let specific_cast = if is_multicast(&self.prefix) {
            &casts.multicast
        } else {
            &casts.unicast
        };
        for entry in [specific_cast, &casts.any].into_iter().flatten() {
            aggregator.join(self.check_entry(dump, entry, accept_num).to_any()?);
        }
        aggregator.to_any()
    }

    pub fn check_entry(&self, dump: &Dump, entry: &Entry, accept_num: Option<usize>) -> AllReport {
        let report = CheckFilter {
            dump,
            compare: self,
            verbosity: self.verbosity,
        }
        .check(&entry.mp_filter, self.recursion_limit)
        .to_all()?;
        match accept_num {
            Some(accept_num) => report.join(
                self.check_peering_actions(dump, &entry.mp_peerings, accept_num)
                    .to_all()?,
            ),
            None => report,
        }
        .to_all()
    }

    pub fn check_peering_actions<'a, I>(
        &self,
        dump: &Dump,
        peerings: I,
        accept_num: usize,
    ) -> AnyReport
    where
        I: IntoIterator<Item = &'a PeeringAction>,
    {
        let mut aggregator = AnyReportAggregator::new();
        for peering_actions in peerings.into_iter() {
            aggregator.join(
                self.check_peering_action(dump, peering_actions, accept_num)
                    .to_any()?,
            );
        }
        aggregator.to_any()
    }

    pub fn check_peering_action(
        &self,
        dump: &Dump,
        peering_actions: &PeeringAction,
        accept_num: usize,
    ) -> AllReport {
        CheckPeering {
            dump,
            compare: self,
            accept_num,
            verbosity: self.verbosity,
        }
        .check(&peering_actions.mp_peering, self.recursion_limit)
        // Skipped.
        /* ?
        .join(self.check_actions(&peering_actions.actions)?)
        .to_all()
        */
    }

    /// We skip community checks, but this could be an enhancement.
    /// <https://github.com/SichangHe/parse_rpsl_policy/issues/16>.
    pub fn check_actions(&self, _actions: &Actions) -> AllReport {
        Ok(None)
    }

    pub fn goes_through_num(&self, num: usize) -> bool {
        self.as_path.iter().any(|p| p.contains_num(num))
    }
}

impl VerbosityReport for Compare {
    fn get_verbosity(&self) -> Verbosity {
        self.verbosity
    }
}

pub const MULTICAST_V4: Result<Ipv4Net, PrefixLenError> =
    Ipv4Net::new(Ipv4Addr::new(224, 0, 0, 0), 4);
pub const MULTICAST_V6: Result<Ipv6Net, PrefixLenError> =
    Ipv6Net::new(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0), 8);

pub fn is_multicast(prefix: &IpNet) -> bool {
    match prefix {
        IpNet::V4(prefix) => MULTICAST_V4
            .expect("MULTICAST_V4 is for sure Ok")
            .contains(prefix),
        IpNet::V6(prefix) => MULTICAST_V6
            .expect("MULTICAST_V6 is for sure Ok")
            .contains(prefix),
    }
}
