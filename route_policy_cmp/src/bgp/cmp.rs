use std::net::{Ipv4Addr, Ipv6Addr};

use anyhow::Result;
use ipnet::{IpNet, Ipv4Net, Ipv6Net, PrefixLenError};

use crate::parse::{
    action::Actions,
    lex::Dump,
    mp_import::{Casts, Entry, Versions},
    peering::{Peering, PeeringAction},
};

use super::{
    filter::CheckFilter,
    map::{parse_table_dump, AsPathEntry},
    report::{
        AllReport, AnyReport, AnyReportAggregater, JoinReportItems, Report, ReportItem,
        ToAllReport, ToAnyReport,
    },
};

pub fn compare_line_w_dump(line: &str, dump: &Dump) -> Result<Vec<Report>> {
    let (prefix, as_path, _, communities) = parse_table_dump(line)?;
    let cmp = Compare::new(dump, prefix, as_path, communities);
    Ok(cmp.check())
}

pub struct Compare<'a> {
    pub dump: &'a Dump,
    pub prefix: IpNet,
    pub as_path: Vec<AsPathEntry>,
    pub communities: Vec<&'a str>,
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
        }
    }
}

impl<'a> Compare<'a> {
    pub fn check(&self) -> Vec<Report> {
        // Iterate the pairs in `as_path` from right to left, with overlaps.
        let pairs = self
            .as_path
            .iter()
            .rev()
            .zip(self.as_path.iter().rev().skip(1));
        pairs
            .flat_map(|(from, to)| {
                if let (AsPathEntry::Seq(from), AsPathEntry::Seq(to)) = (from, to) {
                    self.check_pair(*from, *to)
                } else {
                    vec![Report::skip(format!(
                        "Skipping BGP pair {from}, {to} with set."
                    ))]
                }
            })
            .collect()
    }

    pub fn check_pair(&self, from: usize, to: usize) -> Vec<Report> {
        let from_report = match self.dump.aut_nums.get(&from) {
            Some(from_an) => self.check_compliant(&from_an.exports, to),
            None => Some(Report::skip(format!("{from} is not a recorded AutNum"))),
        };
        let to_report = match self.dump.aut_nums.get(&to) {
            Some(to_an) => self.check_compliant(&to_an.imports, from),
            None => Some(Report::skip(format!("{to} is not a recorded AutNum"))),
        };
        [from_report, to_report].into_iter().flatten().collect()
    }

    pub fn check_compliant(&self, policy: &Versions, accept_num: usize) -> Option<Report> {
        let mut aggregater: AnyReportAggregater = match self.prefix {
            IpNet::V4(_) => self.check_casts(&policy.ipv4, accept_num),
            IpNet::V6(_) => self.check_casts(&policy.ipv6, accept_num),
        }?
        .into();
        aggregater.join(self.check_casts(&policy.any, accept_num)?);
        Some(if aggregater.all_fail {
            aggregater.report_items.push(ReportItem::NoMatch(format!(
                "No policy in {policy:?} matches AS{accept_num} from {}",
                self.prefix
            )));
            Report::Bad(aggregater.report_items)
        } else {
            Report::Neutral(aggregater.report_items)
        })
    }

    pub fn check_casts(&self, casts: &Casts, accept_num: usize) -> AnyReport {
        let mut aggregater = AnyReportAggregater::new();
        let specific_cast = if is_multicast(&self.prefix) {
            &casts.multicast
        } else {
            &casts.unicast
        };
        for entry in [specific_cast, &casts.any].into_iter().flatten() {
            aggregater.join(self.check_entry(entry, accept_num).to_any()?);
        }
        aggregater.to_some()
    }

    pub fn check_entry(&self, entry: &Entry, accept_num: usize) -> AllReport {
        let check_filter = CheckFilter {
            compare: self,
            accept_num,
        };
        Ok(check_filter.check(&entry.mp_filter).to_all()?.join(
            self.check_peering_actions(&entry.mp_peerings, accept_num)
                .to_all()?,
        ))
    }

    pub fn check_peering_actions<I>(&self, peerings: I, accept_num: usize) -> AnyReport
    where
        I: IntoIterator<Item = &'a PeeringAction>,
    {
        let mut aggregater = AnyReportAggregater::new();
        for peering_actions in peerings.into_iter() {
            aggregater.join(
                self.check_peering_action(peering_actions, accept_num)
                    .to_any()?,
            );
        }
        aggregater.to_some()
    }

    pub fn check_peering_action(
        &self,
        peering_actions: &PeeringAction,
        accept_num: usize,
    ) -> AllReport {
        Ok(self
            .check_peering(&peering_actions.mp_peering, accept_num)?
            .join(self.check_actions(&peering_actions.actions)?))
    }

    pub fn check_peering(&self, _peering: &Peering, _accept_num: usize) -> AllReport {
        todo!()
    }

    /// Check communities.
    pub fn check_actions(&self, _actions: &Actions) -> AllReport {
        todo!()
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
