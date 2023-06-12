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
};

pub const RECURSION_LIMIT: isize = 0x100;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Verbosity {
    ErrOnly,
    Brief,
    Detailed,
}

pub struct Compare<'a> {
    pub dump: &'a Dump,
    pub prefix: IpNet,
    pub as_path: Vec<AsPathEntry>,
    pub communities: Vec<&'a str>,
    pub recursion_limit: isize,
    pub verbosity: Verbosity,
}

impl<'a> Compare<'a> {
    pub fn new(
        dump: &'a Dump,
        prefix: IpNet,
        as_path: Vec<AsPathEntry>,
        communities: Vec<&'a str>,
    ) -> Self {
        Self {
            dump,
            prefix,
            as_path,
            communities,
            recursion_limit: RECURSION_LIMIT,
            verbosity: Verbosity::ErrOnly,
        }
    }

    pub fn with_line_dump(line: &'a str, dump: &'a Dump) -> Result<Self> {
        let (prefix, as_path, _, communities) = parse_table_dump(line)?;
        Ok(Self::new(dump, prefix, as_path, communities))
    }

    pub fn check(&self) -> Vec<Report> {
        let mut reports = Vec::new();
        reports.extend(self.check_last_export());

        // Iterate the pairs in `as_path` from right to left, with overlaps.
        let pairs = self
            .as_path
            .iter()
            .rev()
            .zip(self.as_path.iter().rev().skip(1));
        let pair_reports = pairs.flat_map(|(from, to)| {
            if let (AsPathEntry::Seq(from), AsPathEntry::Seq(to)) = (from, to) {
                self.check_pair(*from, *to)
            } else {
                vec![Report::skip(SkipReason::AsPathPairWithSet(
                    from.clone(),
                    to.clone(),
                ))]
            }
        });
        reports.extend(pair_reports);
        reports
    }

    pub fn check_last_export(&self) -> Option<Report> {
        match self.as_path.last() {
            Some(AsPathEntry::Seq(from)) => {
                self.get_aut_num_then(*from, |from_an| self.check_export(from_an, *from, None))
            }
            Some(entry) if self.verbosity > Verbosity::ErrOnly => {
                Some(Report::skip(SkipReason::AsPathWithSet(entry.clone())))
            }
            _ => None,
        }
    }

    pub fn check_pair(&self, from: usize, to: usize) -> Vec<Report> {
        let from_report =
            self.get_aut_num_then(from, |from_an| self.check_export(from_an, from, Some(to)));
        let to_report = self.get_aut_num_then(to, |to_an| self.check_import(to_an, from, to));
        [from_report, to_report].into_iter().flatten().collect()
    }

    pub fn get_aut_num_then<F>(&self, aut_num: usize, call: F) -> Option<Report>
    where
        F: Fn(&AutNum) -> Option<Report>,
    {
        match self.dump.aut_nums.get(&aut_num) {
            Some(aut_num) => call(aut_num),
            None if self.verbosity > Verbosity::ErrOnly => {
                Some(Report::skip(SkipReason::AutNumUnrecorded(aut_num)))
            }
            _ => None,
        }
    }

    pub fn check_export(&self, from_an: &AutNum, from: usize, to: Option<usize>) -> Option<Report> {
        let mut aggregator = self.check_compliant(&from_an.exports, to)?;
        let report = if aggregator.all_fail {
            let reason = match to {
                Some(to) => MatchProblem::NoExportRule(from, to),
                None => MatchProblem::NoExportRuleSingle(from),
            };
            aggregator.join(no_match_any_report(reason).unwrap());
            Report::Bad(aggregator.report_items)
        } else if self.verbosity <= Verbosity::ErrOnly {
            return None;
        } else {
            Report::Neutral(aggregator.report_items)
        };
        Some(report)
    }

    pub fn check_import(&self, to_an: &AutNum, from: usize, to: usize) -> Option<Report> {
        let mut aggregator = self.check_compliant(&to_an.imports, Some(from))?;
        let report = if aggregator.all_fail {
            aggregator.join(no_match_any_report(MatchProblem::NoImportRule(to, from)).unwrap());
            Report::Bad(aggregator.report_items)
        } else if self.verbosity <= Verbosity::ErrOnly {
            return None;
        } else {
            Report::Neutral(aggregator.report_items)
        };
        Some(report)
    }

    pub fn check_compliant(
        &self,
        policy: &Versions,
        accept_num: Option<usize>,
    ) -> Option<AnyReportAggregator> {
        let mut aggregator: AnyReportAggregator = match self.prefix {
            IpNet::V4(_) => self.check_casts(&policy.ipv4, accept_num),
            IpNet::V6(_) => self.check_casts(&policy.ipv6, accept_num),
        }?
        .into();
        aggregator.join(self.check_casts(&policy.any, accept_num)?);
        Some(aggregator)
    }

    pub fn check_casts(&self, casts: &Casts, accept_num: Option<usize>) -> AnyReport {
        let mut aggregator = AnyReportAggregator::new();
        let specific_cast = if is_multicast(&self.prefix) {
            &casts.multicast
        } else {
            &casts.unicast
        };
        for entry in [specific_cast, &casts.any].into_iter().flatten() {
            aggregator.join(self.check_entry(entry, accept_num).to_any()?);
        }
        aggregator.to_any()
    }

    pub fn check_entry(&self, entry: &Entry, accept_num: Option<usize>) -> AllReport {
        let report = CheckFilter {
            compare: self,
            verbosity: self.verbosity,
        }
        .check(&entry.mp_filter, self.recursion_limit)
        .to_all()?;
        match accept_num {
            Some(accept_num) => report.join(
                self.check_peering_actions(&entry.mp_peerings, accept_num)
                    .to_all()?,
            ),
            None => report,
        }
        .to_all()
    }

    pub fn check_peering_actions<I>(&self, peerings: I, accept_num: usize) -> AnyReport
    where
        I: IntoIterator<Item = &'a PeeringAction>,
    {
        let mut aggregator = AnyReportAggregator::new();
        for peering_actions in peerings.into_iter() {
            aggregator.join(
                self.check_peering_action(peering_actions, accept_num)
                    .to_any()?,
            );
        }
        aggregator.to_any()
    }

    pub fn check_peering_action(
        &self,
        peering_actions: &PeeringAction,
        accept_num: usize,
    ) -> AllReport {
        CheckPeering {
            compare: self,
            accept_num,
            verbosity: self.verbosity,
        }
        .check(&peering_actions.mp_peering, self.recursion_limit)?
        .join(self.check_actions(&peering_actions.actions)?)
        .to_all()
    }

    /// Check communities.
    pub fn check_actions(&self, _actions: &Actions) -> AllReport {
        // TODO: We currently do not check actions.
        Ok(None)
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
