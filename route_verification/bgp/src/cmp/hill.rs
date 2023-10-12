use super::*;

impl Compare {
    pub fn check_with_relationship(&self, query: &QueryIr, db: &AsRelDb) -> Vec<Report> {
        let mut reports = self.check(query);
        for report in reports.iter_mut() {
            self.alter_report_with_relationship(report, query, db);
        }
        if !self.verbosity.show_meh {
            reports.retain(|r| !r.is_meh());
        }
        reports
    }

    fn alter_report_with_relationship(&self, report: &mut Report, query: &QueryIr, db: &AsRelDb) {
        match report {
            BadImport { from, to, items } => match db.get(*to, *from) {
                Some(P2C) if self.verbosity.special_uphill => {
                    let reason = match db.is_clique(to) {
                        true => SpecUphillTier1,
                        false => SpecUphill,
                    };
                    *report = self.meh_import(*from, *to, mem::take(items), reason);
                }
                Some(P2P) if self.verbosity.check_import_only_provider => {
                    if let Some(property) = query.as_properties.get(to) {
                        if property.import_only_provider {
                            let reason = SpecImportPeerOIFPS;
                            *report = self.meh_import(*from, *to, mem::take(items), reason);
                        }
                    }
                }
                Some(C2P) if self.verbosity.check_import_only_provider => {
                    if let Some(property) = query.as_properties.get(to) {
                        if property.import_only_provider {
                            let reason = SpecImportCustomerOIFPS;
                            *report = self.meh_import(*from, *to, mem::take(items), reason);
                        }
                    }
                }
                _ if db.is_clique(from) && db.is_clique(to) => {
                    *report = self.meh_import(*from, *to, mem::take(items), SpecTier1Pair);
                }
                _ => (),
            },
            BadExport { from, to, items } => match db.get(*to, *from) {
                Some(P2C) if self.verbosity.special_uphill => {
                    let reason = match db.is_clique(to) {
                        true => SpecUphillTier1,
                        false => SpecUphill,
                    };
                    *report = self.meh_export(*from, *to, mem::take(items), reason);
                }
                _ if db.is_clique(from) && db.is_clique(to) => {
                    *report = self.meh_export(*from, *to, mem::take(items), SpecTier1Pair);
                }
                _ => (),
            },
            _ => (),
        }
    }
}
