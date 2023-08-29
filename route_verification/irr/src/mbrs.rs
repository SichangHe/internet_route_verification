use super::*;

pub fn gather_ref(obj: &RPSLObject, set: &mut BTreeMap<String, BTreeMap<String, Vec<String>>>) {
    let mut member_ofs = Vec::new();
    let mut mnt_by = Vec::new();
    for RpslExpr { key, expr } in expressions(lines_continued(obj.body.lines())) {
        match key.as_str() {
            "member-of" => member_ofs.extend(split_commas(&expr).map(str::to_string)),
            "mnt-by" => mnt_by.extend(split_commas(&expr).map(str::to_string)),
            _ => (),
        }
    }
    for member_of in &member_ofs {
        let referenced_set = set.entry(ref_set(member_of)).or_default();
        for mntner in &mnt_by {
            referenced_set
                .entry(mntner_ref_set(mntner, member_of))
                .or_default()
                .push(obj.name.clone());
        }
    }
}

pub fn conclude_set(set: BTreeMap<String, BTreeMap<String, Vec<String>>>) -> Vec<AsOrRouteSet> {
    let mntner_set_count: usize = set.values().map(BTreeMap::len).sum();
    let mut conclusion = Vec::with_capacity(set.len() + mntner_set_count);
    for (ref_set, mntner_ref_sets) in set {
        let members = mntner_ref_sets.keys().map(Into::into).collect();
        conclusion.push(AsOrRouteSet::new(ref_set, "".into(), members));
        let mntner_ref_sets = mntner_ref_sets
            .into_iter()
            .map(|(name, members)| AsOrRouteSet::new(name, "".into(), members));
        conclusion.extend(mntner_ref_sets);
    }
    conclusion
}

/// Name of pseudo set corresponding to maintainer name `mntner` and
/// `member-of` attribute.
pub fn mntner_ref_set(mntner: &str, member_of: &str) -> String {
    format!("m#{member_of}#{mntner}")
}

/// Name of pseudo set corresponding to `member-of` attribute.
pub fn ref_set(member_of: &str) -> String {
    format!("m#{member_of}")
}
