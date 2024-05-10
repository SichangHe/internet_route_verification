use super::*;

/// List of ASes with all rules compatible with BGPq3.
/// Copy this after running code from [`parse_bgp_lines`].
fn as_compatible_with_bgpq3(query: QueryIr) -> Result<()> {
    fn is_simple(filter: &Filter) -> bool {
        match filter {
            Filter::Any
            | Filter::AsNum(_, _)
            | Filter::AsSet(_, _)
            | Filter::RouteSet(_, _)
            | Filter::AddrPrefixSet(_) => true,
            Filter::FilterSet(_)
            | Filter::AsPathRE(_)
            | Filter::PeerAS
            | Filter::And { left: _, right: _ }
            | Filter::Or { left: _, right: _ }
            | Filter::Not(_)
            | Filter::Group(_)
            | Filter::Community(_)
            | Filter::Unknown(_) => false,
        }
    }

    let ans: Vec<_> = query
        .aut_nums
        .iter()
        .filter(|(_, an)| !an.imports.is_empty() || !an.exports.is_empty())
        .filter(|(_, an)| {
            an.imports.entries_iter().count() == an.n_import as usize
                && an.exports.entries_iter().count() == an.n_export as usize
        })
        .filter(|(_, an)| {
            [&an.imports, &an.exports]
                .into_iter()
                .flat_map(|ports| ports.entries_iter())
                .all(|entry| is_simple(&entry.mp_filter))
        })
        .map(|(num, _)| *num)
        .collect();

    let mut df = DataFrame::new(vec![Series::new("as_compatible_w_bgpq3", ans)])?;
    println!("{df}");
    println!("{}", df.describe(None)?);

    CsvWriter::new(File::create("as_compatible_with_bgpq3.csv")?).finish(&mut df)?;

    Ok(())
}
