use super::*;

pub(crate) fn one(db: &AsRelDb, map: &DashMap<(u64, u64), AsPairStats>, report: Report) {
    let entry = |from, to| {
        map.entry((from, to))
            .or_insert_with(|| AsPairStats::default_with_pair(from, to, db))
    };

    match report {
        GoodImport { from, to } => entry(from, to).import_ok += 1,
        GoodExport { from, to } => entry(from, to).export_ok += 1,
        NeutralImport { from, to, items: _ } => entry(from, to).import_skip += 1,
        NeutralExport { from, to, items: _ } => entry(from, to).export_skip += 1,
        BadImport { from, to, items: _ } => entry(from, to).import_err += 1,
        BadExport { from, to, items: _ } => entry(from, to).export_err += 1,
        AsPathPairWithSet { from: _, to: _ }
        | SetImport { from: _, to: _ }
        | SetExport { from: _, to: _ }
        | SetSingleExport { from: _ }
        | GoodSingleExport { from: _ }
        | NeutralSingleExport { from: _, items: _ }
        | BadSingeExport { from: _, items: _ } => (),
    }
}

/// Using [u32] so it is easy to put into a dataframe later.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AsPairStats {
    pub import_ok: u32,
    pub export_ok: u32,
    pub import_skip: u32,
    pub export_skip: u32,
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
