use super::*;

/// Generate statistics for AS neighbors vs rules.
/// Copy the content after running code from [`parse_bgp_lines`].
fn as_appeared_in_rules(query: QueryIr) {
    use std::collections::HashSet;
    struct Appeared {
        an_list: HashSet<u32>,
        as_set_list: HashSet<String>,
        rs_list: HashSet<String>,
        ps_list: HashSet<String>,
        fs_list: HashSet<String>,
    }
    impl Appeared {
        fn with_query(query: &QueryIr) -> Self {
            Self {
                an_list: HashSet::with_capacity(query.aut_nums.len() * 3 / 2),
                as_set_list: HashSet::with_capacity(query.as_sets.len() * 3 / 2),
                rs_list: HashSet::with_capacity(query.route_sets.len() * 3 / 2),
                ps_list: HashSet::with_capacity(query.peering_sets.len() * 3 / 2),
                fs_list: HashSet::with_capacity(query.filter_sets.len() * 3 / 2),
            }
        }
        fn record_as_expr(&mut self, as_expr: &AsExpr) {
            match as_expr {
                AsExpr::Single(name) => match name {
                    AsName::Num(num) => _ = self.an_list.insert(*num),
                    AsName::Set(name) => _ = self.as_set_list.insert(name.into()),
                    _ => (),
                },
                AsExpr::PeeringSet(name) => _ = self.ps_list.insert(name.into()),
                AsExpr::And { left, right }
                | AsExpr::Or { left, right }
                | AsExpr::Except { left, right } => {
                    self.record_as_expr(left);
                    self.record_as_expr(right);
                }
                AsExpr::Group(as_expr) => self.record_as_expr(as_expr),
            }
        }
        fn record_filter(&mut self, filter: &Filter) {
            match filter {
                Filter::FilterSet(name) => _ = self.fs_list.insert(name.into()),
                Filter::RouteSet(name, _) => _ = self.rs_list.insert(name.into()),
                Filter::AsNum(num, _) => _ = self.an_list.insert(*num),
                Filter::AsSet(name, _) => _ = self.as_set_list.insert(name.into()),
                Filter::And { left, right } | Filter::Or { left, right } => {
                    self.record_filter(left);
                    self.record_filter(right);
                }
                Filter::Not(filter) | Filter::Group(filter) => self.record_filter(filter),
                _ => (),
            }
        }
    }
    let mut appeared = Appeared::with_query(&query);

    for an in query.aut_nums.values() {
        for entry in [&an.imports, &an.exports]
            .into_iter()
            .flat_map(|ports| ports.entries_iter())
        {
            for peering_action in &entry.mp_peerings {
                appeared.record_as_expr(&peering_action.mp_peering.remote_as)
            }
            appeared.record_filter(&entry.mp_filter);
        }
    }

    let Appeared {
        an_list,
        as_set_list,
        rs_list,
        ps_list,
        fs_list,
    } = appeared;
    println!(
        "Appeared in AutNum rules:\t\n{} ASes, {} AS Sets, {} Route Sets, {} Peering Sets, {} Filter Sets.",
        an_list.len(),
        as_set_list.len(),
        rs_list.len(),
        ps_list.len(),
        fs_list.len(),
     );

    let mut df = DataFrame::new(vec![Series::new(
        "aut_nums",
        an_list.into_iter().collect::<Vec<_>>(),
    )])
    .unwrap();
    CsvWriter::new(File::create("aut_nums_appeared_in_rules.csv").unwrap())
        .finish(&mut df)
        .unwrap();

    let mut df = DataFrame::new(vec![Series::new(
        "as_sets",
        as_set_list.into_iter().collect::<Vec<_>>(),
    )])
    .unwrap();
    CsvWriter::new(File::create("as_sets_appeared_in_rules.csv").unwrap())
        .finish(&mut df)
        .unwrap();

    let mut df = DataFrame::new(vec![Series::new(
        "route_sets",
        rs_list.into_iter().collect::<Vec<_>>(),
    )])
    .unwrap();
    CsvWriter::new(File::create("route_sets_appeared_in_rules.csv").unwrap())
        .finish(&mut df)
        .unwrap();

    let mut df = DataFrame::new(vec![Series::new(
        "peering_sets",
        ps_list.into_iter().collect::<Vec<_>>(),
    )])
    .unwrap();
    CsvWriter::new(File::create("peering_sets_appeared_in_rules.csv").unwrap())
        .finish(&mut df)
        .unwrap();

    let mut df = DataFrame::new(vec![Series::new(
        "filter_sets",
        fs_list.into_iter().collect::<Vec<_>>(),
    )])
    .unwrap();
    CsvWriter::new(File::create("filter_sets_appeared_in_rules.csv").unwrap())
        .finish(&mut df)
        .unwrap();
}
