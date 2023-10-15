use super::*;

/// Generate statistics for AS neighbors vs rules.
/// Copy this after running code from [`parse_bgp_lines`].
fn as_neighbors_vs_rules(query: QueryIr, mut bgp_lines: Vec<Line>, db: AsRelDb) -> Result<()> {
    fn n_rules(versions: &Versions) -> u32 {
        versions.entries_iter().count() as u32
    }

    let map: DashMap<u64, (i32, i32, i32)> = DashMap::new();
    db.source2dest.par_iter().for_each(|((as1, as2), _)| {
        map.entry(*as1).or_insert((0, -1, -1)).0 += 1;
        map.entry(*as2).or_insert((0, -1, -1)).0 += 1;
    });

    query.aut_nums.par_iter().for_each(|(num, an)| {
        let mut entry = map.entry(*num).or_insert((-1, 0, 0));
        entry.1 = n_rules(&an.imports) as i32;
        entry.2 = n_rules(&an.exports) as i32;
    });

    let (ans, neighbors, imports, exports): (Vec<u64>, Vec<i32>, Vec<i32>, Vec<i32>) =
        multiunzip(map.into_iter().map(|(an, (nei, im, ex))| (an, nei, im, ex)));
    let mut df = DataFrame::new(vec![
        Series::new("aut_num", ans),
        Series::new("neighbor", neighbors),
        Series::new("import", imports),
        Series::new("export", exports),
    ])?;
    println!("{df}");
    println!("{}", df.describe(None)?);

    CsvWriter::new(File::create("as_neighbors_vs_rules.csv")?).finish(&mut df)?;

    Ok(())
}
