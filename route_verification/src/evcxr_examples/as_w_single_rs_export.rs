use super::*;

/// List of ASes that only export a single Route Set or `Any`.
/// Copy this after running code from [`parse_bgp_lines`].
fn as_with_single_route_set_export(query: QueryIr) -> Result<()> {
    let ans: Vec<_> = query
        .aut_nums
        .iter()
        .filter(|(_, an)| {
            let mut route_set = None;
            for entry in an.exports.entries_iter() {
                match &entry.mp_filter {
                    Filter::Any => (),
                    Filter::RouteSet(rs, _) => match route_set.as_ref() {
                        Some(seen) => {
                            if seen != rs {
                                return false;
                            }
                        }
                        None => route_set = Some(rs.clone()),
                    },
                    _ => return false,
                }
            }
            route_set.is_some()
        })
        .map(|(num, _)| *num)
        .collect();

    let mut df = DataFrame::new(vec![Series::new("as_with_single_route_set_export", ans)])?;
    println!("{df}");
    println!("{}", df.describe(None)?);

    CsvWriter::new(File::create("as_with_single_route_set_export.csv")?).finish(&mut df)?;

    Ok(())
}
