use super::*;

/**
Collect sources for all given ASes.
Copy this whole function to run.
```no_run rust
let (not_found, no_source, sources) = collect_source(&query, &asns);
```
*/
fn collect_source(
    query: &QueryIr,
    asns: &Vec<u32>,
) -> (DashSet<u32>, DashSet<u32>, DashSet<String>) {
    let mut not_found = DashSet::<u32>::new();
    let mut no_source = DashSet::<u32>::new();
    let mut sources = DashSet::<String>::new();
    let record_source = |n: u32, an: &AutNum| {
        for RpslExpr {
            key,
            expr, /*AS*/
        } in expressions(lines_continued(an.body.lines()))
        {
            if let "source" = key.as_str() {
                sources.insert(expr);
                return;
            }
        }
        no_source.insert(n);
    };
    asns.par_iter().for_each(|n| match query.aut_nums.get(n) {
        Some(an) => record_source(*n, an),
        None => _ = not_found.insert(*n),
    });
    (not_found, no_source, sources)
}

/// Generate sources for ASes with unrecorded AutNum.
/// Copy this after running code from [`parse_bgp_lines`].
fn sources_for_as_w_unrec_aut_num(query: QueryIr) -> Result<()> {
    let file_content = read_to_string("as_w_unrec_aut_num.csv")?;
    let asns = file_content
        .lines()
        .skip(1)
        .map(|line| line.trim().parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()?;
    let (not_found, no_source, sources) = collect_source(&query, &asns);

    Ok(())
}
