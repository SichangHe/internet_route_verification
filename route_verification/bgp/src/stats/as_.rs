use super::{
    route::{bad, meh, skip, unrec},
    *,
};

pub fn one(map: &DashMap<u32, RouteStats>, report: Report) {
    match report {
        OkImport { from: _, to } => map.entry(to).or_default().import_ok += 1,
        OkExport { from, to: _ } | OkSingleExport { from } => {
            map.entry(from).or_default().export_ok += 1
        }
        SkipImport { from: _, to, items } => {
            let mut entry = map.entry(to).or_default();
            entry.import_skip += 1;
            skip(&mut entry, items);
        }
        SkipExport { from, to: _, items } | SkipSingleExport { from, items } => {
            let mut entry = map.entry(from).or_default();
            entry.export_skip += 1;
            skip(&mut entry, items);
        }
        UnrecImport { from: _, to, items } => {
            let mut entry = map.entry(to).or_default();
            entry.import_unrec += 1;
            unrec(&mut entry, items);
        }
        UnrecExport { from, to: _, items } | UnrecSingleExport { from, items } => {
            let mut entry = map.entry(from).or_default();
            entry.export_unrec += 1;
            unrec(&mut entry, items);
        }
        BadImport { from: _, to, items } => {
            let mut entry = map.entry(to).or_default();
            entry.import_err += 1;
            bad(&mut entry, items);
        }
        BadExport { from, to: _, items } | BadSingleExport { from, items } => {
            let mut entry = map.entry(from).or_default();
            entry.export_err += 1;
            bad(&mut entry, items);
        }
        MehImport { from: _, to, items } => {
            let mut entry = map.entry(to).or_default();
            entry.import_meh += 1;
            meh(&mut entry, items);
        }
        MehExport { from, to: _, items } | MehSingleExport { from, items } => {
            let mut entry = map.entry(from).or_default();
            entry.export_meh += 1;
            meh(&mut entry, items);
        }
        AsPathPairWithSet { from: _, to: _ } | SetSingleExport { from: _ } => (),
    }
}
