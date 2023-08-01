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
                .contains(&AsPathEntry::Seq(65000))
                .then(|| line.clone())
        })
        .collect();
    println!("{}", filtered_bgp_lines.len());
    filtered_bgp_lines.par_iter_mut().for_each(|line| {
        line.compare.verbosity = Verbosity::minimum_all();
        line.report = Some(line.compare.check_with_relationship(&query, &db))
    });

    for line in &filtered_bgp_lines[..10] {
        line.display();
    }

    let mut all_display = filtered_bgp_lines
        .par_iter()
        .map(|line| line.display_str())
        .collect::<Vec<_>>()
        .join("\n");
    File::create("AS65000_reports.txt")?.write_all(all_display.as_bytes());

    let mut all_non_skip = filtered_bgp_lines
        .par_iter()
        .filter_map(|line| {
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
                        } | Report::SkipSingleExport { from: _, items: _ }
                    )
                })
                .then(|| line.display_str())
        })
        .collect::<Vec<_>>()
        .join("\n");
    File::create("AS65000_non_skip_reports.txt")?.write_all(all_non_skip.as_bytes());

    let mut line = filtered_bgp_lines[0].clone();

    Ok(())
}
