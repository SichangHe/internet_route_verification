use super::*;

/// Check a specific line.
/// Copy this after running code from [`parse_bgp_lines`].
fn as_neighbors_vs_rules(query: QueryDump, db: AsRelDb) -> Result<()> {
    let mut line: Line = Line::from_raw("TABLE_DUMP2|1687212000|B|85.114.0.217|8492|1.32.0.0/17|8492 6939 4788|IGP|85.114.0.217|0|0|8492:1208 8492:1601|NAG||".into())?;
    line.compare.verbosity = Verbosity {
        all_err: true,
        ..Verbosity::minimum_all()
    };
    line.report = Some(line.compare.check_with_relationship(&query, &db));
    line.display();

    Ok(())
}
