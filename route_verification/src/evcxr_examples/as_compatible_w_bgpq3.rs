use super::*;

/// List of ASes with all rules compatible with BGPq3.
/// Copy this after running code from [`parse_bgp_lines`].
fn as_compatible_with_bgpq3(query: QueryIr) -> Result<()> {
    fn is_simple(filter: &Filter) -> bool {
        match filter {
            Filter::Any | Filter::AsNum(_, _) | Filter::AsSet(_, _) | Filter::RouteSet(_, _) => {
                true
            }
            Filter::FilterSet(_) => todo!(),
            Filter::AddrPrefixSet(_) => todo!(),
            Filter::AsPathRE(_)
            | Filter::And { left: _, right: _ }
            | Filter::Or { left: _, right: _ }
            | Filter::Not(_)
            | Filter::Group(_)
            | Filter::Community(_)
            | Filter::Unknown(_)
            | Filter::Invalid(_) => false,
        }
    }

    let ans: Vec<_> = query
        .aut_nums
        .iter()
        .filter(|(_, an)| {
            [&an.imports, &an.exports]
                .into_iter()
                .flat_map(|ports| ports.entries_iter())
                .all(|entry| is_simple(&entry.mp_filter))
        })
        .map(|(num, _)| *num)
        .collect();

    Ok(())
}
