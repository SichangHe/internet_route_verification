use super::*;

impl Compare {
    pub fn check_hill(&self, dump: &QueryDump, db: &AsRelDb) -> Vec<Report> {
        let mut reports = self.check(dump);
        for report in reports.iter_mut() {
            match report {
                BadImport { from, to, items } => {
                    if let Some(P2C) = db.get(*to, *from) {
                        let mut items = mem::take(items);
                        if self.verbosity.show_meh {
                            items.push(Special(SpecialCase::Uphill))
                        }
                        *report = MehImport {
                            from: *from,
                            to: *to,
                            items,
                        };
                    }
                }
                BadExport { from, to, items } => {
                    if let Some(P2C) = db.get(*to, *from) {
                        let mut items = mem::take(items);
                        if self.verbosity.show_meh {
                            items.push(Special(SpecialCase::Uphill))
                        }
                        *report = MehExport {
                            from: *from,
                            to: *to,
                            items,
                        }
                    }
                }
                _ => (),
            }
        }
        reports
    }
}
