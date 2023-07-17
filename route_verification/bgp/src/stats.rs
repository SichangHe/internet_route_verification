use dashmap::DashMap;

use super::*;

use Report::*;

impl Compare {
    pub fn as_stats(&mut self, dump: &QueryDump, map: &DashMap<u64, AsStats>) {
        self.verbosity = Verbosity {
            stop_at_first: false,
            show_skips: true,
            show_success: true,
            ..Verbosity::default()
        };
        let reports = self.check(dump);
        for report in reports {
            match report {
                GoodImport { from: _, to } => map.entry(to).or_default().import_ok += 1,
                GoodExport { from, to: _ } | GoodSingleExport { from } => {
                    map.entry(from).or_default().export_ok += 1
                }
                NeutralImport {
                    from: _,
                    to,
                    items: _,
                } => map.entry(to).or_default().import_skip += 1,
                NeutralExport {
                    from,
                    to: _,
                    items: _,
                }
                | NeutralSingleExport { from, items: _ } => {
                    map.entry(from).or_default().export_skip += 1
                }
                BadImport {
                    from: _,
                    to,
                    items: _,
                } => map.entry(to).or_default().import_err += 1,
                BadExport {
                    from,
                    to: _,
                    items: _,
                }
                | BadSingeExport { from, items: _ } => map.entry(from).or_default().export_err += 1,
                _ => (),
            }
        }
    }
}

/// Using [u32] so it is easy to put into a dataframe later.
#[derive(Clone, Debug, Default)]
pub struct AsStats {
    pub import_ok: u32,
    pub export_ok: u32,
    pub import_skip: u32,
    pub export_skip: u32,
    pub import_err: u32,
    pub export_err: u32,
}

pub fn up_down_hill_stats<'a, I>(reports_iter: I, db: &AsRelDb)
where
    I: IntoIterator<Item = &'a Vec<Report>>,
{
    let mut result = UpDownHillStats::default();
    for reports in reports_iter {
        for report in reports {
            match report {
                GoodImport { from, to } => match db.get(from, to) {
                    Some(P2C) => todo!(),
                    Some(P2P) => todo!(),
                    Some(C2P) => todo!(),
                    None => todo!(),
                },
                GoodExport { from, to } => todo!(),
                GoodSingleExport { from } => todo!(),
                NeutralImport { from, to, items } => todo!(),
                NeutralExport { from, to, items } => todo!(),
                NeutralSingleExport { from, items } => todo!(),
                AsPathPairWithSet { from, to } => todo!(),
                SetImport { from, to } => todo!(),
                SetExport { from, to } => todo!(),
                SetSingleExport { from } => todo!(),
                BadImport { from, to, items } => todo!(),
                BadExport { from, to, items } => todo!(),
                BadSingeExport { from, items } => todo!(),
            }
        }
    }
}

/// Using [u32] so it is easy to put into a dataframe later.
#[derive(Clone, Debug, Default)]
pub struct UpDownHillStats {
    pub good_up_import: u32,
    pub good_down_import: u32,
    pub good_peer_import: u32,
    pub good_other_import: u32,
    pub good_up_export: u32,
    pub good_down_export: u32,
    pub good_peer_export: u32,
    pub good_other_export: u32,
    pub neutral_up_import: u32,
    pub neutral_down_import: u32,
    pub neutral_peer_import: u32,
    pub neutral_other_import: u32,
    pub neutral_up_export: u32,
    pub neutral_down_export: u32,
    pub neutral_peer_export: u32,
    pub neutral_other_export: u32,
    pub bad_up_import: u32,
    pub bad_down_import: u32,
    pub bad_peer_import: u32,
    pub bad_other_import: u32,
    pub bad_up_export: u32,
    pub bad_down_export: u32,
    pub bad_peer_export: u32,
    pub bad_other_export: u32,
}
