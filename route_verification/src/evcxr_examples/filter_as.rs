use super::*;

fn reports_for_paths_containing_certain_as(
    an: u32,
    query: &QueryIr,
    bgp_lines: &[Line],
    db: &AsRelDb,
) {
    let mut filtered_bgp_lines: Vec<&Line> = bgp_lines
        .par_iter()
        .filter_map(|line| {
            line.compare
                .as_path
                .contains(&AsPathEntry::Seq(an))
                .then_some(line)
        })
        .collect();
    println!("{}", filtered_bgp_lines.len());

    let mut target = File::create(format!("AS{an}_non_skip_reports.txt")).unwrap();

    for chunk in filtered_bgp_lines.chunks(256) {
        let mut all_non_skip = chunk
            .par_iter()
            .filter_map(|line| {
                let mut line = (*line).clone();
                line.compare.verbosity = Verbosity {
                    all_err: true,
                    record_set: true,
                    record_community: true,
                    ..Verbosity::minimum_all()
                };
                line.report = Some(line.compare.check_with_relationship(query, db));
                line.report
                    .as_ref()
                    .unwrap()
                    .iter()
                    .any(|report| {
                        !matches!(
                            report,
                            Report::SkipImport {
                                from: _,
                                to: _,
                                items: _,
                            } | Report::SkipExport {
                                from: _,
                                to: _,
                                items: _,
                            }
                        )
                    })
                    .then(|| line.display_str())
            })
            .collect::<Vec<_>>()
            .join("\n");
        all_non_skip.push('\n');
        target.write_all(all_non_skip.as_bytes());
    }
}
