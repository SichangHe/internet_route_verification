use super::*;

pub fn one(stats: &mut UpDownHillStats, report: &Report, db: &AsRelDb) {
    match report {
        GoodImport { from, to } => match db.get(*from, *to) {
            Some(P2C) => stats.good_down_import += 1,
            Some(P2P) => stats.good_peer_import += 1,
            Some(C2P) => stats.good_up_import += 1,
            None => stats.good_other_import += 1,
        },
        GoodExport { from, to } => match db.get(*from, *to) {
            Some(P2C) => stats.good_down_export += 1,
            Some(P2P) => stats.good_peer_export += 1,
            Some(C2P) => stats.good_up_export += 1,
            None => stats.good_other_export += 1,
        },
        GoodSingleExport { from: _ } => stats.good_other_export += 1,
        NeutralImport { from, to, items: _ } => match db.get(*from, *to) {
            Some(P2C) => stats.neutral_down_import += 1,
            Some(P2P) => stats.neutral_peer_import += 1,
            Some(C2P) => stats.neutral_up_import += 1,
            None => stats.neutral_other_import += 1,
        },
        NeutralExport { from, to, items: _ } => match db.get(*from, *to) {
            Some(P2C) => stats.neutral_down_export += 1,
            Some(P2P) => stats.neutral_peer_export += 1,
            Some(C2P) => stats.neutral_up_export += 1,
            None => stats.neutral_other_export += 1,
        },
        NeutralSingleExport { from: _, items: _ } => stats.neutral_other_export += 1,
        BadImport { from, to, items: _ } => match db.get(*from, *to) {
            Some(P2C) => stats.bad_down_import += 1,
            Some(P2P) => stats.bad_peer_import += 1,
            Some(C2P) => stats.bad_up_import += 1,
            None => stats.bad_other_import += 1,
        },
        BadExport { from, to, items: _ } => match db.get(*from, *to) {
            Some(P2C) => stats.bad_down_export += 1,
            Some(P2P) => stats.bad_peer_export += 1,
            Some(C2P) => stats.bad_up_export += 1,
            None => stats.bad_other_export += 1,
        },
        BadSingeExport { from: _, items: _ } => stats.bad_other_export += 1,
        AsPathPairWithSet { from: _, to: _ } | SetSingleExport { from: _ } => (),
    }
}

/// Using [u32] so it is easy to put into a dataframe later.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

impl UpDownHillStats {
    pub fn sum(&self) -> u32 {
        self.good_up_import
            + self.good_down_import
            + self.good_peer_import
            + self.good_other_import
            + self.good_up_export
            + self.good_down_export
            + self.good_peer_export
            + self.good_other_export
            + self.neutral_up_import
            + self.neutral_down_import
            + self.neutral_peer_import
            + self.neutral_other_import
            + self.neutral_up_export
            + self.neutral_down_export
            + self.neutral_peer_export
            + self.neutral_other_export
            + self.bad_up_import
            + self.bad_down_import
            + self.bad_peer_import
            + self.bad_other_import
            + self.bad_up_export
            + self.bad_down_export
            + self.bad_peer_export
            + self.bad_other_export
    }
}

impl Add for UpDownHillStats {
    type Output = UpDownHillStats;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            good_up_import: self.good_up_import + rhs.good_up_import,
            good_down_import: self.good_down_import + rhs.good_down_import,
            good_peer_import: self.good_peer_import + rhs.good_peer_import,
            good_other_import: self.good_other_import + rhs.good_other_import,
            good_up_export: self.good_up_export + rhs.good_up_export,
            good_down_export: self.good_down_export + rhs.good_down_export,
            good_peer_export: self.good_peer_export + rhs.good_peer_export,
            good_other_export: self.good_other_export + rhs.good_other_export,
            neutral_up_import: self.neutral_up_import + rhs.neutral_up_import,
            neutral_down_import: self.neutral_down_import + rhs.neutral_down_import,
            neutral_peer_import: self.neutral_peer_import + rhs.neutral_peer_import,
            neutral_other_import: self.neutral_other_import + rhs.neutral_other_import,
            neutral_up_export: self.neutral_up_export + rhs.neutral_up_export,
            neutral_down_export: self.neutral_down_export + rhs.neutral_down_export,
            neutral_peer_export: self.neutral_peer_export + rhs.neutral_peer_export,
            neutral_other_export: self.neutral_other_export + rhs.neutral_other_export,
            bad_up_import: self.bad_up_import + rhs.bad_up_import,
            bad_down_import: self.bad_down_import + rhs.bad_down_import,
            bad_peer_import: self.bad_peer_import + rhs.bad_peer_import,
            bad_other_import: self.bad_other_import + rhs.bad_other_import,
            bad_up_export: self.bad_up_export + rhs.bad_up_export,
            bad_down_export: self.bad_down_export + rhs.bad_down_export,
            bad_peer_export: self.bad_peer_export + rhs.bad_peer_export,
            bad_other_export: self.bad_other_export + rhs.bad_other_export,
        }
    }
}
