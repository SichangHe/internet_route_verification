use anyhow::Result;
use ipnet::IpNet;

use crate::parse::{
    action::Actions,
    filter::Filter,
    lex::Dump,
    mp_import::{Casts, Entry, Versions},
    peering::{Peering, PeeringAction},
};

use super::map::{parse_table_dump, AsPathEntry};

pub fn compare_line_w_dump(line: &str, dump: &Dump) -> Result<Vec<Report>> {
    let (prefix, as_path, _, communities) = parse_table_dump(line)?;
    let cmp = Compare::new(dump, prefix, as_path, communities);
    Ok(cmp.check())
}

pub struct Compare<'a> {
    dump: &'a Dump,
    prefix: IpNet,
    as_path: Vec<AsPathEntry>,
    communities: Vec<&'a str>,
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
                    self.pair_pair(*from, *to)
                } else {
                    vec![Report::Skip(format!(
                        "Skipping BGP pair {from}, {to} with set."
                    ))]
                }
            })
            .collect()
    }

    pub fn pair_pair(&self, from: usize, to: usize) -> Vec<Report> {
        let mut from_report = match self.dump.aut_nums.get(&from) {
            Some(from_an) => self.check_compliant(&from_an.exports, to),
            None => vec![Report::Skip(format!("{from} is not a recorded AutNum"))],
        };
        let to_report = match self.dump.aut_nums.get(&to) {
            Some(to_an) => self.check_compliant(&to_an.imports, from),
            None => vec![Report::Skip(format!("{to} is not a recorded AutNum"))],
        };
        from_report.extend(to_report);
        from_report
    }

    pub fn check_compliant(&self, policy: &Versions, accept_num: usize) -> Vec<Report> {
        let mut reports = Vec::new();
        let specific_report = match self.prefix {
            IpNet::V4(_) => self.check_casts(&policy.ipv4, accept_num),
            IpNet::V6(_) => self.check_casts(&policy.ipv6, accept_num),
        };
        if let Some(Report::Good) = specific_report.first() {
            // This route is good.
            return specific_report;
        }
        reports.extend(specific_report);

        let general_report = self.check_casts(&policy.any, accept_num);
        if let Some(Report::Good) = general_report.first() {
            // This route is good.
            return general_report;
        }
        reports.extend(general_report);

        if reports.is_empty() {
            reports.push(Report::NoMatch(format!(
                "No policy in {policy:?} matches AS{accept_num} from {}",
                self.prefix
            )));
        }
        reports
    }

    pub fn check_casts(&self, casts: &Casts, accept_num: usize) -> Vec<Report> {
        let mut reports = Vec::new();
        // TODO: How do we know the casts?
        for entry in [&casts.multicast, &casts.unicast, &casts.any]
            .into_iter()
            .flatten()
        {
            let report = self.check_entry(entry, accept_num);
            if let Some(Report::Good) = report.first() {
                // This route is good.
                return vec![Report::Good];
            }
            reports.extend(report);
        }
        reports
    }

    pub fn check_entry(&self, entry: &Entry, accept_num: usize) -> Vec<Report> {
        let mut reports = Vec::new();
        match self.check_filter(&entry.mp_filter, accept_num) {
            Some(filter_report) => reports.push(filter_report),
            None => return vec![],
        }
        for peering_actions in &entry.mp_peerings {
            match self.check_peering_actions(peering_actions, accept_num) {
                Some(Report::Good) => return vec![Report::Good],
                Some(report) => reports.push(report),
                None => (),
            }
        }
        reports
    }

    pub fn check_filter(&self, filter: &Filter, accept_num: usize) -> Option<Report> {
        todo!()
    }

    pub fn check_peering_actions(
        &self,
        peering_actions: &PeeringAction,
        accept_num: usize,
    ) -> Option<Report> {
        self.check_peering(&peering_actions.mp_peering, accept_num)
            .filter(|_| self.check_actions(&peering_actions.actions))
    }

    pub fn check_peering(&self, peering: &Peering, accept_num: usize) -> Option<Report> {
        todo!()
    }

    /// Check communities.
    pub fn check_actions(&self, actions: &Actions) -> bool {
        todo!()
    }
}

pub enum Report {
    Skip(String),
    Good,
    NoMatch(String),
}
