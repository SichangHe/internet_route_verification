use super::*;

/// Number of rules compatible with BGPq3 for each AS.
/// Copy the content after running code from [`parse_bgp_lines`].
fn bgpq3_compatible_rules(query: QueryIr) {
    fn is_simple(filter: &Filter) -> bool {
        match filter {
            Filter::Any
            | Filter::AsNum(_, _)
            | Filter::AsSet(_, _)
            | Filter::RouteSet(_, _)
            | Filter::AddrPrefixSet(_) => true,
            Filter::FilterSet(_)
            | Filter::AsPathRE(_)
            | Filter::PeerAS
            | Filter::And { left: _, right: _ }
            | Filter::Or { left: _, right: _ }
            | Filter::Not(_)
            | Filter::Group(_)
            | Filter::Community(_)
            | Filter::Unknown(_) => false,
        }
    }

    let mut file = BufWriter::new(File::create("bgpq3_compatible_rules.csv").unwrap());
    file.write_all(b"aut_num,import,export,simple_import,simple_export\n")
        .unwrap();

    for (as_num, aut_num) in &query.aut_nums {
        file.write_all(as_num.to_string().as_bytes()).unwrap();
        file.write_all(b",").unwrap();
        file.write_all(aut_num.n_import.to_string().as_bytes())
            .unwrap();
        file.write_all(b",").unwrap();
        file.write_all(aut_num.n_export.to_string().as_bytes())
            .unwrap();
        file.write_all(b",").unwrap();

        let n_simple_import = aut_num
            .imports
            .entries_iter()
            .filter(|entry| is_simple(&entry.mp_filter))
            .count();
        file.write_all(n_simple_import.to_string().as_bytes())
            .unwrap();
        file.write_all(b",").unwrap();

        let n_simple_export = aut_num
            .exports
            .entries_iter()
            .filter(|entry| is_simple(&entry.mp_filter))
            .count();
        file.write_all(n_simple_export.to_string().as_bytes())
            .unwrap();
        file.write_all(b"\n").unwrap();
    }

    file.flush().unwrap();
    drop(file);
}
