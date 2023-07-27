use super::*;

/// Extract customers for each AutNum into a set under the name `c#{aut_num}`.
pub fn make_customer_pseudo_set(db: &AsRelDb) -> BTreeMap<String, AsSet> {
    db.source2dest
        .par_iter()
        .fold(
            BTreeMap::<_, Vec<_>>::new,
            |mut result, ((from, to), relationship)| {
                match *relationship {
                    P2C => result.entry(*from).or_default().push(*to),
                    P2P => (),
                    C2P => result.entry(*to).or_default().push(*from),
                }
                result
            },
        )
        .reduce(BTreeMap::new, |a, b| {
            let (mut large, small) = if a.len() > b.len() { (a, b) } else { (b, a) };
            for (key, value) in small {
                large.entry(key).or_default().extend(value);
            }
            large
        })
        .into_par_iter()
        .map(|(an, customers)| {
            (
                format!("c#{an}"),
                AsSet {
                    body: "".into(),
                    members: customers,
                    set_members: vec![],
                },
            )
        })
        .collect()
}
