use super::*;

/// Count `filter`s with `community`
/// Copy the content after running code from [`parse_bgp_lines`],
fn count_community_filter(query: QueryIr) -> Result<()> {
    let count = query
        .aut_nums
        .values()
        .flat_map(|aut_num| [&aut_num.imports, &aut_num.exports])
        .flat_map(|port| port.entries_iter())
        .filter(|entry| matches!(entry.mp_filter, Filter::Community(_)))
        .count();

    Ok(())
}
