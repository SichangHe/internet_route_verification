use super::*;

pub fn one(map: &DashMap<u64, AsStats>, report: Report) {
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
        | NeutralSingleExport { from, items: _ } => map.entry(from).or_default().export_skip += 1,
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
        AsPathPairWithSet { from: _, to: _ } | SetSingleExport { from: _ } => (),
    }
}

/// Using [u32] so it is easy to put into a dataframe later.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AsStats {
    pub import_ok: u32,
    pub export_ok: u32,
    pub import_skip: u32,
    pub export_skip: u32,
    pub import_err: u32,
    pub export_err: u32,
}
