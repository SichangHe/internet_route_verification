use super::*;

pub(crate) fn one(db: &AsRelDb, map: &DashMap<(u64, u64), AsPairStats>, report: Report) {
    let entry = |from, to| {
        map.entry((from, to))
            .or_insert_with(|| AsPairStats::default_with_pair(from, to, db))
    };

    match report {
        OkImport { from, to } => entry(from, to).import_ok += 1,
        OkExport { from, to } => entry(from, to).export_ok += 1,
        SkipImport { from, to, items: _ } => entry(from, to).import_skip += 1,
        SkipExport { from, to, items: _ } => entry(from, to).export_skip += 1,
        UnrecImport { from, to, items: _ } => entry(from, to).import_unrec += 1,
        UnrecExport { from, to, items: _ } => entry(from, to).export_unrec += 1,
        BadImport { from, to, items: _ } => entry(from, to).import_err += 1,
        BadExport { from, to, items: _ } => entry(from, to).export_err += 1,
        MehImport { from, to, items: _ } => entry(from, to).import_meh += 1,
        MehExport { from, to, items: _ } => entry(from, to).export_meh += 1,
        AsPathPairWithSet { from: _, to: _ }
        | SetSingleExport { from: _ }
        | OkSingleExport { from: _ }
        | SkipSingleExport { from: _, items: _ }
        | UnrecSingleExport { from: _, items: _ }
        | MehSingleExport { from: _, items: _ }
        | BadSingleExport { from: _, items: _ } => (),
    }
}

/// Using [u32] so it is easy to put into a dataframe later.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AsPairStats {
    pub import_ok: u32,
    pub export_ok: u32,
    pub import_skip: u32,
    pub export_skip: u32,
    pub import_unrec: u32,
    pub export_unrec: u32,
    pub import_meh: u32,
    pub export_meh: u32,
    pub import_err: u32,
    pub export_err: u32,
    pub relationship: Option<Relationship>,
}

impl AsPairStats {
    pub fn default_with_pair(from: u64, to: u64, db: &AsRelDb) -> Self {
        Self {
            relationship: db.get(from, to),
            ..Self::default()
        }
    }
}
