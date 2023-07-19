use super::*;

impl Compare {
    pub fn check_hill(&self, dump: &QueryDump, db: &AsRelDb) -> Vec<Report> {
        let mut reports = self.check(dump);
        for report in reports.iter_mut() {
            match report {
                BadImport { from, to, items } => {
                    if let Some(P2C) = db.get(*to, *from) {
                        *report = BadImportUp {
                            from: *from,
                            to: *to,
                            items: mem::take(items),
                        };
                    }
                }
                BadExport { from, to, items } => {
                    if let Some(P2C) = db.get(*to, *from) {
                        *report = BadExportUp {
                            from: *from,
                            to: *to,
                            items: mem::take(items),
                        }
                    }
                }
                _ => (),
            }
        }
        reports
    }
}
