use super::*;

pub fn one(map: &DashMap<u32, RouteStats<u64>>, report: Report) {
    match report {
        OkImport { from: _, to } => map.entry(to).or_default().import_ok += 1,
        OkExport { from, to: _ } => map.entry(from).or_default().export_ok += 1,
        SkipImport { from: _, to, items } => {
            let mut entry = map.entry(to).or_default();
            entry.import_skip += 1;
            entry.skip(items);
        }
        SkipExport { from, to: _, items } => {
            let mut entry = map.entry(from).or_default();
            entry.export_skip += 1;
            entry.skip(items);
        }
        UnrecImport { from: _, to, items } => {
            let mut entry = map.entry(to).or_default();
            entry.import_unrec += 1;
            entry.unrec(items);
        }
        UnrecExport { from, to: _, items } => {
            let mut entry = map.entry(from).or_default();
            entry.export_unrec += 1;
            entry.unrec(items);
        }
        BadImport { from: _, to, items } => {
            let mut entry = map.entry(to).or_default();
            entry.import_err += 1;
            entry.bad(items);
        }
        BadExport { from, to: _, items } => {
            let mut entry = map.entry(from).or_default();
            entry.export_err += 1;
            entry.bad(items);
        }
        MehImport { from: _, to, items } => {
            let mut entry = map.entry(to).or_default();
            entry.import_meh += 1;
            entry.meh(items);
        }
        MehExport { from, to: _, items } => {
            let mut entry = map.entry(from).or_default();
            entry.export_meh += 1;
            entry.meh(items);
        }
        AsPathPairWithSet { from: _, to: _ } => (),
    }
}
