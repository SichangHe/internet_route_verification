use super::*;

/// Count how many rules have router information.
/// Copy this after running code from [`parse_bgp_lines`].
fn count_rules_w_router_info(query: QueryIr) -> Result<()> {
    let start = Instant::now();
    let count: usize = query
        .aut_nums
        .par_iter()
        .map(|(_, an)| {
            [&an.imports, &an.exports]
                .iter()
                .flat_map(|versions| versions.entries_iter())
                .filter(|entry| {
                    entry.mp_peerings.iter().any(|p| {
                        p.mp_peering.local_router.is_some() || p.mp_peering.remote_router.is_some()
                    })
                })
                .count()
        })
        .sum();
    println!(
        "Found {count} rule entries in {}ms.",
        start.elapsed().as_millis()
    );

    Ok(())
}
