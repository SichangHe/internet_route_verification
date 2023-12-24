use super::*;

pub fn one(db: &AsRelDb, map: &DashMap<(u32, u32), AsPairStats>, report: &Report) {
    let entry = |from, to| {
        map.entry((from, to))
            .or_insert_with(|| AsPairStats::default_with_pair(from, to, db))
    };

    match report {
        OkImport { from, to } => entry(*from, *to).route_stats.import_ok += 1,
        OkExport { from, to } => entry(*from, *to).route_stats.export_ok += 1,
        SkipImport { from, to, items } => {
            let mut entry = entry(*from, *to);
            entry.route_stats.import_skip += 1;
            entry.route_stats.skip(items)
        }
        SkipExport { from, to, items } => {
            let mut entry = entry(*from, *to);
            entry.route_stats.export_skip += 1;
            entry.route_stats.skip(items)
        }
        UnrecImport { from, to, items } => {
            let mut entry = entry(*from, *to);
            entry.route_stats.import_unrec += 1;
            entry.route_stats.unrec(items)
        }
        UnrecExport { from, to, items } => {
            let mut entry = entry(*from, *to);
            entry.route_stats.export_unrec += 1;
            entry.route_stats.unrec(items)
        }
        BadImport { from, to, items } => {
            let mut entry = entry(*from, *to);
            entry.route_stats.import_err += 1;
            entry.route_stats.bad(items)
        }
        BadExport { from, to, items } => {
            let mut entry = entry(*from, *to);
            entry.route_stats.export_err += 1;
            entry.route_stats.bad(items)
        }
        MehImport { from, to, items } => {
            let mut entry = entry(*from, *to);
            entry.route_stats.import_meh += 1;
            entry.route_stats.meh(items);
        }
        MehExport { from, to, items } => {
            let mut entry = entry(*from, *to);
            entry.route_stats.export_meh += 1;
            entry.route_stats.meh(items);
        }
        AsPathPairWithSet { from: _, to: _ } => (),
    }
}

/// Using [u32] so it is easy to put into a dataframe later.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AsPairStats {
    pub route_stats: RouteStats<u64>,
    pub relationship: Option<Relationship>,
}

impl AsPairStats {
    pub fn default_with_pair(from: u32, to: u32, db: &AsRelDb) -> Self {
        Self {
            relationship: db.get(from, to),
            ..Self::default()
        }
    }
}
