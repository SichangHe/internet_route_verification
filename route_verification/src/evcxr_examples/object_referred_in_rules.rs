use super::*;

/// RPSL objects referred to in AutNum rules.
/// Copy this after running code from [`parse_bgp_lines`].
fn object_referred_in_rules(query: QueryIr) {
    use std::collections::HashMap;
    #[derive(Debug, Default)]
    struct Appearance {
        recorded: bool,
        import_peering: usize,
        export_peering: usize,
        import_filter: usize,
        export_filter: usize,
        import_overall: usize,
        export_overall: usize,
    }

    impl Appearance {
        fn recorded() -> Self {
            Self {
                recorded: true,
                ..Self::default()
            }
        }

        fn unrecorded() -> Self {
            Self {
                recorded: false,
                ..Self::default()
            }
        }
    }

    #[derive(Debug, Default)]
    struct Appeared {
        an_list: Vec<u32>,
        as_set_list: Vec<String>,
        rs_list: Vec<String>,
        ps_list: Vec<String>,
        fs_list: Vec<String>,
    }

    impl Appeared {
        fn collect_as_expr(&mut self, as_expr: &AsExpr) {
            match as_expr {
                AsExpr::Single(name) => match name {
                    AsName::Num(num) => self.an_list.push(*num),
                    AsName::Set(set) => self.as_set_list.push(set.into()),
                    _ => {}
                },
                AsExpr::PeeringSet(set) => self.ps_list.push(set.into()),
                AsExpr::And { left, right }
                | AsExpr::Or { left, right }
                | AsExpr::Except { left, right } => {
                    self.collect_as_expr(left);
                    self.collect_as_expr(right)
                }
                AsExpr::Group(as_expr) => self.collect_as_expr(as_expr),
            }
        }

        fn collect_filter(&mut self, filter: &Filter) {
            match filter {
                Filter::FilterSet(set) => self.fs_list.push(set.into()),
                Filter::RouteSet(set, _) => self.rs_list.push(set.into()),
                Filter::AsNum(num, _) => self.an_list.push(*num),
                Filter::AsSet(set, _) => self.as_set_list.push(set.into()),
                Filter::AsPathRE(regex) => self.collect_as_path_regex(regex),
                Filter::And { left, right } | Filter::Or { left, right } => {
                    self.collect_filter(left);
                    self.collect_filter(right);
                }
                Filter::Not(filter) | Filter::Group(filter) => self.collect_filter(filter),
                _ => {}
            }
        }

        fn collect_as_path_regex(&mut self, regex: &str) {
            use route_verification::{
                bgp::cmp::as_path_regex::char_map::*,
                common_regex::{regex, Replacer},
            };
            let mut as_sets = CharMap::<String>::new_from_alpha();
            let mut as_nums = CharMap::<u32>::new_from_alpha();
            let regex = as_set_replace_all(regex, as_sets.by_ref());
            let _ = as_replace_all(&regex, as_sets.by_ref());
            self.as_set_list.extend(as_sets.char_map);
            self.an_list.extend(as_nums.char_map);
        }

        fn clean_up(&mut self) {
            macro_rules! clean_up_vecs {
                // For each expression passed in, run Vec::sort and Vec::dedup
                ($($vec:expr),*) => {
                    $(
                        $vec.sort_unstable();
                        $vec.dedup();
                    )*
                }
            }
            clean_up_vecs!(
                self.an_list,
                self.as_set_list,
                self.rs_list,
                self.ps_list,
                self.fs_list
            );
        }
    }

    struct QueryRecord {
        an_list: HashMap<u32, Appearance>,
        as_set_list: HashMap<String, Appearance>,
        rs_list: HashMap<String, Appearance>,
        ps_list: HashMap<String, Appearance>,
        fs_list: HashMap<String, Appearance>,
    }

    impl QueryRecord {
        fn add_rule(&mut self, entry: &Entry, is_export: bool) {
            let mut as_expr_appeared = Appeared::default();
            entry
                .mp_peerings
                .iter()
                .map(|peering_action| &peering_action.mp_peering.remote_as)
                .map(|as_expr| {
                    as_expr_appeared.collect_as_expr(as_expr);
                });
            as_expr_appeared.clean_up();

            macro_rules! add_peerings {
                ($from_list:expr, $to_list:expr) => {
                    for x in $from_list {
                        let appearance = $to_list
                            .entry(x.clone())
                            .or_insert(Appearance::unrecorded());
                        if is_export {
                            appearance.export_peering += 1;
                        } else {
                            appearance.import_peering += 1;
                        }
                    }
                };
            }
            add_peerings!(&as_expr_appeared.an_list, self.an_list);
            add_peerings!(&as_expr_appeared.as_set_list, self.as_set_list);
            add_peerings!(&as_expr_appeared.rs_list, self.rs_list);
            add_peerings!(&as_expr_appeared.ps_list, self.ps_list);
            add_peerings!(&as_expr_appeared.fs_list, self.fs_list);

            let mut filter_appeared = Appeared::default();
            filter_appeared.collect_filter(&entry.mp_filter);
            filter_appeared.clean_up();

            macro_rules! add_filters {
                ($from_list:expr, $to_list:expr) => {
                    for x in $from_list {
                        let appearance = $to_list
                            .entry(x.clone())
                            .or_insert(Appearance::unrecorded());
                        if is_export {
                            appearance.export_filter += 1;
                        } else {
                            appearance.import_filter += 1;
                        }
                    }
                };
            }
            add_filters!(&filter_appeared.an_list, self.an_list);
            add_filters!(&filter_appeared.as_set_list, self.as_set_list);
            add_filters!(&filter_appeared.rs_list, self.rs_list);
            add_filters!(&filter_appeared.ps_list, self.ps_list);
            add_filters!(&filter_appeared.fs_list, self.fs_list);

            as_expr_appeared.an_list.extend(filter_appeared.an_list);
            as_expr_appeared
                .as_set_list
                .extend(filter_appeared.as_set_list);
            as_expr_appeared.rs_list.extend(filter_appeared.rs_list);
            as_expr_appeared.ps_list.extend(filter_appeared.ps_list);
            as_expr_appeared.fs_list.extend(filter_appeared.fs_list);
            as_expr_appeared.clean_up();

            macro_rules! add_overall {
                ($from_list:expr, $to_list:expr) => {
                    for x in $from_list {
                        let appearance = $to_list
                            .entry(x.clone())
                            .or_insert(Appearance::unrecorded());
                        if is_export {
                            appearance.export_overall += 1;
                        } else {
                            appearance.import_overall += 1;
                        }
                    }
                };
            }
            add_overall!(&as_expr_appeared.an_list, self.an_list);
            add_overall!(&as_expr_appeared.as_set_list, self.as_set_list);
            add_overall!(&as_expr_appeared.rs_list, self.rs_list);
            add_overall!(&as_expr_appeared.ps_list, self.ps_list);
            add_overall!(&as_expr_appeared.fs_list, self.fs_list);
        }
    }

    let mut query_record = QueryRecord {
        an_list: query
            .aut_nums
            .par_keys()
            .map(|num| (*num, Appearance::recorded()))
            .collect(),
        as_set_list: query
            .as_sets
            .par_keys()
            .filter(|name| !name.contains('#'))
            .map(|name| (name.clone(), Appearance::recorded()))
            .collect(),
        rs_list: query
            .route_sets
            .par_keys()
            .map(|name| (name.clone(), Appearance::recorded()))
            .collect(),
        ps_list: query
            .peering_sets
            .par_keys()
            .map(|name| (name.clone(), Appearance::recorded()))
            .collect(),
        fs_list: query
            .filter_sets
            .par_keys()
            .map(|name| (name.clone(), Appearance::recorded()))
            .collect(),
    };

    query.aut_nums.values().for_each(|an| {
        for entry in an.imports.entries_iter() {
            query_record.add_rule(entry, false);
        }
        for entry in an.exports.entries_iter() {
            query_record.add_rule(entry, true);
        }
    });

    {
        let mut file = BufWriter::new(File::create("as_num_appearances_in_rules.csv").unwrap());
        file.write_all(b"as_num,recorded,import_peering,export_peering,import_filter,export_filter,import_overall,export_overall\n").unwrap();
        for (num, appearance) in &query_record.an_list {
            writeln!(
                file,
                "{},{},{},{},{},{},{},{}",
                num,
                appearance.recorded,
                appearance.import_peering,
                appearance.export_peering,
                appearance.import_filter,
                appearance.export_filter,
                appearance.import_overall,
                appearance.export_overall
            )
            .unwrap();
        }
    }

    {
        let mut file = BufWriter::new(File::create("as_set_appearances_in_rules.csv").unwrap());
        file.write_all(b"as_set,recorded,import_peering,export_peering,import_filter,export_filter,import_overall,export_overall\n").unwrap();
        for (set, appearance) in &query_record.as_set_list {
            writeln!(
                file,
                "{},{},{},{},{},{},{},{}",
                set,
                appearance.recorded,
                appearance.import_peering,
                appearance.export_peering,
                appearance.import_filter,
                appearance.export_filter,
                appearance.import_overall,
                appearance.export_overall
            )
            .unwrap();
        }
    }

    {
        let mut file = BufWriter::new(File::create("route_set_appearances_in_rules.csv").unwrap());
        file.write_all(b"route_set,recorded,import_peering,export_peering,import_filter,export_filter,import_overall,export_overall\n").unwrap();
        for (set, appearance) in &query_record.rs_list {
            writeln!(
                file,
                "{},{},{},{},{},{},{},{}",
                set,
                appearance.recorded,
                appearance.import_peering,
                appearance.export_peering,
                appearance.import_filter,
                appearance.export_filter,
                appearance.import_overall,
                appearance.export_overall
            )
            .unwrap();
        }
    }

    {
        let mut file =
            BufWriter::new(File::create("peering_set_appearances_in_rules.csv").unwrap());
        file.write_all(b"peering_set,recorded,import_peering,export_peering,import_filter,export_filter,import_overall,export_overall\n").unwrap();
        for (set, appearance) in &query_record.ps_list {
            writeln!(
                file,
                "{},{},{},{},{},{},{},{}",
                set,
                appearance.recorded,
                appearance.import_peering,
                appearance.export_peering,
                appearance.import_filter,
                appearance.export_filter,
                appearance.import_overall,
                appearance.export_overall
            )
            .unwrap();
        }
    }

    {
        let mut file = BufWriter::new(File::create("filter_set_appearances_in_rules.csv").unwrap());
        file.write_all(b"filter_set,recorded,import_peering,export_peering,import_filter,export_filter,import_overall,export_overall\n").unwrap();
        for (set, appearance) in &query_record.fs_list {
            writeln!(
                file,
                "{},{},{},{},{},{},{},{}",
                set,
                appearance.recorded,
                appearance.import_peering,
                appearance.export_peering,
                appearance.import_filter,
                appearance.export_filter,
                appearance.import_overall,
                appearance.export_overall
            )
            .unwrap();
        }
    }
}
