use super::*;

/// Check a specific line.
/// Copy the content after running code from [`parse_bgp_lines`].
fn check_specific_line(query: QueryIr, db: AsRelDb) {
    let mut line: Line = Line::from_raw("TABLE_DUMP2|1687212004|B|89.149.178.10|3257|45.161.144.0/23|3257 3356 28186 268199 268510 268510|IGP|89.149.178.10|0|10|3257:8794 3257:30043 3257:50001 3257:54900 3257:54901|NAG||".into()).unwrap();
    line.compare.verbosity = Verbosity {
        per_peering_err: true,
        ..Verbosity::minimum_all()
    };
    line.report = Some(line.compare.check_with_relationship(&query, &db));
    line.display();
}
