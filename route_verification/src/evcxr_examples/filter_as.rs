use super::*;

fn reports_for_paths_containing_certain_as(
    query: QueryDump,
    mut bgp_lines: Vec<Line>,
    db: AsRelDb,
) -> Result<()> {
    let mut filtered_bgp_lines: Vec<Line> = bgp_lines
        .par_iter()
        .filter_map(|line| {
            line.compare
                .as_path
                .contains(&AsPathEntry::Seq(139609))
                .then(|| line.clone())
        })
        .collect();
    println!("{}", filtered_bgp_lines.len());
    filtered_bgp_lines.par_iter_mut().for_each(|line| {
        line.compare.verbosity = Verbosity::minimum_all();
        line.report = Some(line.compare.check_hill(&query, &db))
    });

    for line in &filtered_bgp_lines[..10] {
        line.display();
    }

    let mut all_display = filtered_bgp_lines
        .par_iter()
        .map(|line| line.display_str())
        .collect::<Vec<_>>()
        .join("\n");
    File::create("AS139609_reports.txt")?.write_all(all_display.as_bytes());

    let mut all_non_neutral = filtered_bgp_lines
        .par_iter()
        .filter_map(|line| {
            line.report
                .as_ref()
                .unwrap()
                .iter()
                .any(|report| {
                    !matches!(
                        report,
                        Report::NeutralImport {
                            from: _,
                            to: _,
                            items: _,
                        } | Report::NeutralExport {
                            from: _,
                            to: _,
                            items: _,
                        } | Report::NeutralSingleExport { from: _, items: _ }
                    )
                })
                .then(|| line.display_str())
        })
        .collect::<Vec<_>>()
        .join("\n");
    File::create("AS139609_non_neutral_reports.txt")?.write_all(all_non_neutral.as_bytes());

    let mut line = filtered_bgp_lines[0].clone();

    Ok(())
}
