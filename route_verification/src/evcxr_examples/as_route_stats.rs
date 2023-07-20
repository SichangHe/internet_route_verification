use super::*;

/// Generate statistics for AS neighbors vs rules.
/// Copy this after running code from [`parse_bgp_lines`].
fn as_neighbors_vs_rules(query: QueryDump, mut bgp_lines: Vec<Line>, db: AsRelDb) {
    fn n_rules(versions: &Versions) -> u32 {
        let Versions { any, ipv4, ipv6 } = versions;
        [any, ipv4, ipv6]
            .into_iter()
            .map(|casts| {
                let Casts {
                    any,
                    unicast,
                    multicast,
                } = casts;
                [any, unicast, multicast]
                    .into_iter()
                    .map(Vec::len)
                    .sum::<usize>()
            })
            .sum::<usize>() as u32
    }

    let map: DashMap<&u64, (i32, i32, i32)> = DashMap::new();
    db.source2dest.par_iter().for_each(|((as1, as2), _)| {
        map.entry(as1).or_insert((0, 0, 0)).0 += 1;
        map.entry(as2).or_insert((0, 0, 0)).0 += 1;
    });
}
