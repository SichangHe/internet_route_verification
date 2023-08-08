use super::*;

impl Compare {
    pub fn check_with_relationship(&self, dump: &QueryDump, db: &AsRelDb) -> Vec<Report> {
        let mut reports = self.check(dump);
        for report in reports.iter_mut() {
            self.alter_report_with_relationship(report, dump, db);
        }
        if !self.verbosity.show_meh {
            reports.retain(|r| !r.is_meh());
        }
        reports
    }

    fn alter_report_with_relationship(&self, report: &mut Report, dump: &QueryDump, db: &AsRelDb) {
        match report {
            BadImport { from, to, items } => match db.get(*to, *from) {
                Some(P2C) => {
                    *report = self.meh_import(*from, *to, mem::take(items), Uphill);
                }
                Some(P2P) if self.verbosity.check_import_only_provider => {
                    if let Some(property) = dump.as_properties.get(to) {
                        if property.import_only_provider {
                            let reason = P2PWOnlyP2CImport;
                            *report = self.meh_import(*from, *to, mem::take(items), reason);
                        }
                    }
                }
                _ if db.is_clique(from) && db.is_clique(to) => {
                    *report = self.meh_import(*from, *to, mem::take(items), Tier1Pair);
                }
                _ => (),
            },
            BadExport { from, to, items } => {
                if let Some(P2C) = db.get(*to, *from) {
                    *report = self.meh_export(*from, *to, mem::take(items), Uphill);
                } else if db.is_clique(from) && db.is_clique(to) {
                    *report = self.meh_export(*from, *to, mem::take(items), Tier1Pair);
                }
            }
            _ => (),
        }
    }
}
