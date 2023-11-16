use super::*;

pub fn one(stats: &mut UpDownHillStats, report: &Report, db: &AsRelDb) {
    match report {
        OkImport { from, to } => match db.get(*from, *to) {
            Some(P2C) => stats.ok_down_import += 1,
            Some(P2P) => stats.ok_peer_import += 1,
            Some(C2P) => stats.ok_up_import += 1,
            None => stats.ok_other_import += 1,
        },
        OkExport { from, to } => match db.get(*from, *to) {
            Some(P2C) => stats.ok_down_export += 1,
            Some(P2P) => stats.ok_peer_export += 1,
            Some(C2P) => stats.ok_up_export += 1,
            None => stats.ok_other_export += 1,
        },
        SkipImport { from, to, items: _ } | UnrecImport { from, to, items: _ } => {
            match db.get(*from, *to) {
                Some(P2C) => stats.skip_down_import += 1,
                Some(P2P) => stats.skip_peer_import += 1,
                Some(C2P) => stats.skip_up_import += 1,
                None => stats.skip_other_import += 1,
            }
        }
        SkipExport { from, to, items: _ } | UnrecExport { from, to, items: _ } => {
            match db.get(*from, *to) {
                Some(P2C) => stats.skip_down_export += 1,
                Some(P2P) => stats.skip_peer_export += 1,
                Some(C2P) => stats.skip_up_export += 1,
                None => stats.skip_other_export += 1,
            }
        }
        BadImport { from, to, items: _ } | MehImport { from, to, items: _ } => {
            match db.get(*from, *to) {
                Some(P2C) => stats.bad_down_import += 1,
                Some(P2P) => stats.bad_peer_import += 1,
                Some(C2P) => stats.bad_up_import += 1,
                None => stats.bad_other_import += 1,
            }
        }
        BadExport { from, to, items: _ } | MehExport { from, to, items: _ } => {
            match db.get(*from, *to) {
                Some(P2C) => stats.bad_down_export += 1,
                Some(P2P) => stats.bad_peer_export += 1,
                Some(C2P) => stats.bad_up_export += 1,
                None => stats.bad_other_export += 1,
            }
        }
        AsPathPairWithSet { from: _, to: _ } => (),
    }
}

/// Using [u32] so it is easy to put into a dataframe later.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UpDownHillStats {
    pub ok_up_import: u32,
    pub ok_down_import: u32,
    pub ok_peer_import: u32,
    pub ok_other_import: u32,
    pub ok_up_export: u32,
    pub ok_down_export: u32,
    pub ok_peer_export: u32,
    pub ok_other_export: u32,
    pub skip_up_import: u32,
    pub skip_down_import: u32,
    pub skip_peer_import: u32,
    pub skip_other_import: u32,
    pub skip_up_export: u32,
    pub skip_down_export: u32,
    pub skip_peer_export: u32,
    pub skip_other_export: u32,
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
        self.ok_up_import
            + self.ok_down_import
            + self.ok_peer_import
            + self.ok_other_import
            + self.ok_up_export
            + self.ok_down_export
            + self.ok_peer_export
            + self.ok_other_export
            + self.skip_up_import
            + self.skip_down_import
            + self.skip_peer_import
            + self.skip_other_import
            + self.skip_up_export
            + self.skip_down_export
            + self.skip_peer_export
            + self.skip_other_export
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
            ok_up_import: self.ok_up_import + rhs.ok_up_import,
            ok_down_import: self.ok_down_import + rhs.ok_down_import,
            ok_peer_import: self.ok_peer_import + rhs.ok_peer_import,
            ok_other_import: self.ok_other_import + rhs.ok_other_import,
            ok_up_export: self.ok_up_export + rhs.ok_up_export,
            ok_down_export: self.ok_down_export + rhs.ok_down_export,
            ok_peer_export: self.ok_peer_export + rhs.ok_peer_export,
            ok_other_export: self.ok_other_export + rhs.ok_other_export,
            skip_up_import: self.skip_up_import + rhs.skip_up_import,
            skip_down_import: self.skip_down_import + rhs.skip_down_import,
            skip_peer_import: self.skip_peer_import + rhs.skip_peer_import,
            skip_other_import: self.skip_other_import + rhs.skip_other_import,
            skip_up_export: self.skip_up_export + rhs.skip_up_export,
            skip_down_export: self.skip_down_export + rhs.skip_down_export,
            skip_peer_export: self.skip_peer_export + rhs.skip_peer_export,
            skip_other_export: self.skip_other_export + rhs.skip_other_export,
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
