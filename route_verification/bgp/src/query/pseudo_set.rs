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
        .map(|(aut_num, customers)| {
            (
                customer_set(aut_num),
                AsSet::new("".into(), customers, vec![]),
            )
        })
        .collect()
}

/// Name of the customer pseudo set corresponding to `aut_num`.
pub fn customer_set(aut_num: u32) -> String {
    format!("c#{aut_num}")
}
