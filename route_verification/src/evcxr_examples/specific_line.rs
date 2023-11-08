use super::*;

/// Check a specific line.
/// Copy this after running code from [`parse_bgp_lines`].
fn as_neighbors_vs_rules(query: QueryIr, db: AsRelDb) -> Result<()> {
    let mut line: Line = Line::from_raw("TABLE_DUMP2|1687212004|B|89.149.178.10|3257|45.161.144.0/23|3257 3356 28186 268199 268510 268510|IGP|89.149.178.10|0|10|3257:8794 3257:30043 3257:50001 3257:54900 3257:54901|NAG||".into())?;
    line.compare.verbosity = Verbosity {
        per_entry_err: true,
        ..Verbosity::minimum_all()
    };
    line.report = Some(line.compare.check_with_relationship(&query, &db));
    line.display();

    Ok(())
}
