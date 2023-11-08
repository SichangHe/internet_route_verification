use super::*;

/// Generate statistics for AS pairs.
/// Copy this after running code from [`parse_bgp_lines`].
fn gen_as_pair_stats(query: QueryIr, mut bgp_lines: Vec<Line>, db: AsRelDb) -> Result<()> {
    let start = Instant::now();
    let map: DashMap<(u32, u32), AsPairStats> = DashMap::new();
    bgp_lines.par_iter_mut().for_each(|l| {
        l.compare.as_pair_stats(&query, &db, &map);
    });
    let size = map.len();
    println!(
        "Generated stats of {size} AS pairs in {}ms.",
        start.elapsed().as_millis()
    );

    let (from_tos, ioks, eoks, isps, esps, iurs, eurs, imhs, emhs, iers, eers, rels): (
        Vec<(u32, u32)>,
        Vec<u32>,
        Vec<u32>,
        Vec<u32>,
        Vec<u32>,
        Vec<u32>,
        Vec<u32>,
        Vec<u32>,
        Vec<u32>,
        Vec<u32>,
        Vec<u32>,
        Vec<String>,
    ) = multiunzip(map.into_iter().map(
        |(
            (from, to),
            AsPairStats {
                import_ok,
                export_ok,
                import_skip,
                export_skip,
                import_unrec,
                export_unrec,
                import_meh,
                export_meh,
                import_err,
                export_err,
                relationship,
            },
        )| {
            (
                (from, to),
                import_ok,
                export_ok,
                import_skip,
                export_skip,
                import_unrec,
                export_unrec,
                import_meh,
                export_meh,
                import_err,
                export_err,
                String::from(match relationship {
                    Some(Relationship::P2C) => "down",
                    Some(Relationship::P2P) => "peer",
                    Some(Relationship::C2P) => "up",
                    None => "other",
                }),
            )
        },
    ));
    let (froms, tos): (Vec<u32>, Vec<u32>) = multiunzip(from_tos);

    let mut df: DataFrame = DataFrame::new(vec![
        Series::new("from", froms),
        Series::new("to", tos),
        Series::new("import_ok", ioks),
        Series::new("export_ok", eoks),
        Series::new("import_skip", isps),
        Series::new("export_skip", esps),
        Series::new("import_unrec", iurs),
        Series::new("export_unrec", eurs),
        Series::new("import_meh", imhs),
        Series::new("export_meh", emhs),
        Series::new("import_err", iers),
        Series::new("export_err", eers),
        Series::new("relationship", rels),
    ])?;
    println!("{df}");
    println!("{}", df.describe(None)?);

    CsvWriter::new(File::create("as_pair_stats.csv")?).finish(&mut df)?;

    Ok(())
}

/// Generate statistics for up/downhill.
/// Copy this after running code from [`parse_bgp_lines`].
fn gen_up_down_hill_stats(query: QueryIr, mut bgp_lines: Vec<Line>, db: AsRelDb) -> Result<()> {
    let start = Instant::now();
    let up_down_hill_stats: UpDownHillStats = bgp_lines
        .par_iter_mut()
        .map(|l| l.compare.up_down_hill_stats(&query, &db))
        .reduce(UpDownHillStats::default, Add::add);
    let total = up_down_hill_stats.sum();
    println!(
        "Generated stats of {total} reports in {}ms.",
        start.elapsed().as_millis()
    );

    let mut up_down_hill_df: DataFrame = DataFrame::new(vec![
        Series::new(
            "quality",
            vec![
                "ok", "ok", "ok", "ok", "ok", "ok", "ok", "ok", "skip", "skip", "skip", "skip",
                "skip", "skip", "skip", "skip", "bad", "bad", "bad", "bad", "bad", "bad", "bad",
                "bad",
            ],
        ),
        Series::new(
            "hill",
            vec![
                "up", "down", "peer", "other", "up", "down", "peer", "other", "up", "down", "peer",
                "other", "up", "down", "peer", "other", "up", "down", "peer", "other", "up",
                "down", "peer", "other",
            ],
        ),
        Series::new(
            "port",
            vec![
                "import", "import", "import", "import", "export", "export", "export", "export",
                "import", "import", "import", "import", "export", "export", "export", "export",
                "import", "import", "import", "import", "export", "export", "export", "export",
            ],
        ),
        Series::new(
            "value",
            vec![
                up_down_hill_stats.ok_up_import,
                up_down_hill_stats.ok_down_import,
                up_down_hill_stats.ok_peer_import,
                up_down_hill_stats.ok_other_import,
                up_down_hill_stats.ok_up_export,
                up_down_hill_stats.ok_down_export,
                up_down_hill_stats.ok_peer_export,
                up_down_hill_stats.ok_other_export,
                up_down_hill_stats.skip_up_import,
                up_down_hill_stats.skip_down_import,
                up_down_hill_stats.skip_peer_import,
                up_down_hill_stats.skip_other_import,
                up_down_hill_stats.skip_up_export,
                up_down_hill_stats.skip_down_export,
                up_down_hill_stats.skip_peer_export,
                up_down_hill_stats.skip_other_export,
                up_down_hill_stats.bad_up_import,
                up_down_hill_stats.bad_down_import,
                up_down_hill_stats.bad_peer_import,
                up_down_hill_stats.bad_other_import,
                up_down_hill_stats.bad_up_export,
                up_down_hill_stats.bad_down_export,
                up_down_hill_stats.bad_peer_export,
                up_down_hill_stats.bad_other_export,
            ],
        ),
    ])?;
    CsvWriter::new(File::create("up_down_hill_stats.csv")?).finish(&mut up_down_hill_df)?;

    Ok(())
}

/// Generate statistics for each AS.
/// Copy this after running code from [`parse_bgp_lines`],
fn gen_as_stats(query: QueryIr, mut bgp_lines: Vec<Line>, db: AsRelDb) -> Result<()> {
    let start = Instant::now();
    let map: DashMap<u32, RouteStats<u32>> = DashMap::new();
    bgp_lines.par_iter_mut().for_each(|l| {
        l.compare.as_stats(&query, &db, &map);
    });
    let size = map.len();
    println!(
        "Generated stats for {size} AS in {}ms.",
        start.elapsed().as_millis()
    );

    let mut file = BufWriter::new(File::create("as_stats.csv")?);
    file.write_all(csv_header().trim_end_matches(',').as_bytes());
    file.write_all(b"\n");
    for (an, s) in map.into_iter() {
        file.write_all(an.to_string().as_bytes());
        file.write_all(b",");
        file.write_all(&s.as_csv_bytes());
        file.write_all(b"\n");
    }
    file.flush()?;
    drop(file);

    Ok(())
}
