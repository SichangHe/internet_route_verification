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
    filtered_bgp_lines
        .par_iter_mut()
        .for_each(|line| line.report = Some(line.compare.check_hill(&query, &db)));

    for line in &filtered_bgp_lines[..10] {
        line.display();
    }

    let mut all_display = filtered_bgp_lines
        .par_iter()
        .map(|line| line.display_str())
        .collect::<Vec<_>>()
        .join("\n");
    File::create("AS139609_reports.txt")?.write_all(all_display.as_bytes());

    Ok(())
}
