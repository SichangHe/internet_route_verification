use super::*;

/// Count how many sets are in AS-paths of `bgp_lines`.
/// Copy this after running code from [`parse_bgp_lines`].
fn gen_route_stats(bgp_lines: Vec<Line>) -> Result<()> {
    let start = Instant::now();
    let count: usize = bgp_lines
        .par_iter()
        .map(|line| {
            line.compare
                .as_path
                .iter()
                .filter(|a| !matches!(a, AsPathEntry::Seq(_)))
                .count()
        })
        .sum();
    println!("Found {count} sets in {}ms.", start.elapsed().as_millis());

    Ok(())
}
