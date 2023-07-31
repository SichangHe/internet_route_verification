use super::*;

/// Check a specific line.
/// Copy this after running code from [`parse_bgp_lines`].
fn as_neighbors_vs_rules(query: QueryDump, db: AsRelDb) -> Result<()> {
    let mut line: Line = Line::from_raw("TABLE_DUMP2|1687212013|B|105.16.0.247|37100|103.2.88.0/24|37100 6939 7545 17559 139609 45891 134525|IGP|105.16.0.247|0|0|no-export|NAG||".into())?;
    line.compare.verbosity = Verbosity {
        all_err: true,
        ..Verbosity::minimum_all()
    };
    line.report = Some(line.compare.check_hill(&query, &db));
    line.display();

    Ok(())
}
