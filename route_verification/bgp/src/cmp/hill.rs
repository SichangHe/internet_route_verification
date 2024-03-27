use super::*;

impl Compare {
    /// Same as [`check`](#method.check), except that AS Relationship DB `db` is used to
    /// convert suitable "bad" reports to "meh".
    /// - If `self.verbosity.show_meh` is `false`,
    /// then these "meh" reports are removed.
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
        let (from, to, items, this_as, is_export) = match report {
            BadImport { from, to, items } => (*from, *to, items, *to, false),
            BadExport { from, to, items } => (*from, *to, items, *from, true),
            _ => return,
        };
        let only_provider_policies =
            || {
                self.verbosity.check_only_provider_policies
                    && query.as_properties.get(&this_as).map(|property| {
                        property.import_only_provider && property.export_only_provider
                    }) == Some(true)
            };
        let maybe_report_reason = match (db.get(from, to), is_export) {
            (Some(C2P), _) if self.verbosity.special_uphill => Some(match db.is_clique(&to) {
                true => SpecUphillTier1,
                false => SpecUphill,
            }),
            (Some(P2P), _) if only_provider_policies() => Some(SpecPeerOnlyProviderPolicies),
            (Some(C2P), false) | (Some(P2C), true) if only_provider_policies() => {
                Some(SpecCustomerOnlyProviderPolicies)
            }
            _ if db.is_clique(&from) && db.is_clique(&to) => Some(SpecTier1Pair),
            _ => None,
        };
        if let Some(reason) = maybe_report_reason {
            *report = match is_export {
                false => self.meh_import(from, to, mem::take(items), reason),
                true => self.meh_export(from, to, mem::take(items), reason),
            }
        }
    }
}
