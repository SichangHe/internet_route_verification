use super::*;

/// Generate sources for ASes with unrecorded AutNum.
/// Copy the whole function after running code from [`parse_bgp_lines`],
/// and run as
/// ```no_run
/// transit_as_rules(&query, &db)
/// ```
fn transit_as_rules(query: &QueryIr, db: &AsRelDb) -> Result<()> {
    let mut transit_ases: Vec<u32> = db
        .source2dest
        .iter()
        .filter_map(|((from, to), relationship)| match relationship {
            Relationship::P2C => Some(*from),
            Relationship::P2P => None,
            Relationship::C2P => Some(*to),
        })
        .collect();
    transit_ases.sort_unstable();
    transit_ases.dedup();

    #[derive(Default)]
    struct Appear {
        as_num: u32,
        overall: Vec<u32>,
        peering: Vec<u32>,
        filter: Vec<u32>,
    }

    impl Appear {
        fn record_as_expr(&mut self, as_expr: &AsExpr, query: &QueryIr) {
            match as_expr {
                AsExpr::Single(AsName::Num(num)) => {
                    self.overall.push(*num);
                    self.peering.push(*num);
                }
                AsExpr::Single(AsName::Set(name)) => {
                    if let Some(as_set) = query.as_sets.get(name) {
                        self.overall.extend(&as_set.members);
                        self.peering.extend(&as_set.members);
                    }
                }
                AsExpr::Single(AsName::Any | AsName::Invalid(_)) => {}
                AsExpr::PeeringSet(name) => {
                    if let Some(peering_set) = query.peering_sets.get(name) {
                        for peering in &peering_set.peerings {
                            self.record_as_expr(&peering.remote_as, query);
                        }
                    }
                }
                AsExpr::And { left, right }
                | AsExpr::Or { left, right }
                | AsExpr::Except { left, right } => {
                    self.record_as_expr(left, query);
                    self.record_as_expr(right, query);
                }
                AsExpr::Group(as_expr) => self.record_as_expr(as_expr, query),
            }
        }
        fn record_filter(&mut self, filter: &Filter, query: &QueryIr) {
            match filter {
                Filter::FilterSet(name) => {
                    if let Some(filter_set) = query.filter_sets.get(name) {
                        for filter in &filter_set.filters {
                            self.record_filter(filter, query);
                        }
                    }
                }
                Filter::AsNum(num, _) => {
                    self.overall.push(*num);
                    self.filter.push(*num);
                }
                Filter::AsSet(name, _) => {
                    if let Some(as_set) = query.as_sets.get(name) {
                        self.overall.extend(&as_set.members);
                        self.filter.extend(&as_set.members)
                    }
                }
                Filter::And { left, right } | Filter::Or { left, right } => {
                    self.record_filter(left, query);
                    self.record_filter(right, query);
                }
                Filter::Not(filter) | Filter::Group(filter) => self.record_filter(filter, query),
                Filter::Any
                | Filter::AddrPrefixSet(_)
                | Filter::RouteSet(_, _)
                | Filter::AsPathRE(_)
                | Filter::PeerAS
                | Filter::Community(_)
                | Filter::Unknown(_) => {}
            }
        }
        fn record_entry(&mut self, entry: &Entry, query: &QueryIr) {
            for peering_action in &entry.mp_peerings {
                self.record_as_expr(&peering_action.mp_peering.remote_as, query);
            }
            self.record_filter(&entry.mp_filter, query);
        }
        fn clean_up(&mut self) {
            self.overall.sort_unstable();
            self.overall.dedup();
            self.peering.sort_unstable();
            self.peering.dedup();
            self.filter.sort_unstable();
            self.filter.dedup();
        }
    }

    let mut file = BufWriter::new(File::create("transit_as_stats2.csv")?);
    file.write_all(b"as_num,import_provider,import_peer,import_customer,import_other,import_both_provider,import_both_peer,import_both_customer,import_both_other,import_peering_provider,import_filter_provider,import_peering_peer,import_filter_peer,import_peering_customer,import_filter_customer,import_peering_other,import_filter_other,export_provider,export_peer,export_customer,export_other,export_self,export_peering_provider,export_filter_provider,export_peering_peer,export_filter_peer,export_peering_customer,export_filter_customer,export_peering_other,export_filter_other,export_peering_self,export_filter_self\n");

    macro_rules! write_comma_num {
        ($num:expr) => {
            file.write_all(b",")?;
            file.write_all($num.to_string().as_bytes())?;
        };
    }

    for &as_num in &transit_ases {
        file.write_all(as_num.to_string().as_bytes())?;
        let Some(aut_num) = query.aut_nums.get(&as_num) else {
            file.write_all(b",-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1\n");
            continue;
        };

        {
            let mut import_provider = 0;
            let mut import_peer = 0;
            let mut import_customer = 0;
            let mut import_other = 0;
            let mut import_both_provider = 0;
            let mut import_both_peer = 0;
            let mut import_both_customer = 0;
            let mut import_both_other = 0;
            let mut import_peering_provider = 0;
            let mut import_filter_provider = 0;
            let mut import_peering_peer = 0;
            let mut import_filter_peer = 0;
            let mut import_peering_customer = 0;
            let mut import_filter_customer = 0;
            let mut import_peering_other = 0;
            let mut import_filter_other = 0;

            let mut appear = Appear {
                as_num,
                ..Default::default()
            };
            for entry in aut_num.imports.entries_iter() {
                appear.record_entry(entry, query);
            }
            appear.clean_up();

            for import_as in appear.overall {
                match db.get(as_num, import_as) {
                    Some(Relationship::C2P) => import_provider += 1,
                    Some(Relationship::P2P) => import_peer += 1,
                    Some(Relationship::P2C) => import_customer += 1,
                    None => import_other += 1,
                }
            }

            for &import_as in &appear.peering {
                if appear.filter.contains(&import_as) {
                    match db.get(as_num, import_as) {
                        Some(Relationship::C2P) => import_both_provider += 1,
                        Some(Relationship::P2P) => import_both_peer += 1,
                        Some(Relationship::P2C) => import_both_customer += 1,
                        None => import_peering_other += 1,
                    }
                }
            }

            for import_as in appear.peering {
                match db.get(as_num, import_as) {
                    Some(Relationship::C2P) => import_peering_provider += 1,
                    Some(Relationship::P2P) => import_peering_peer += 1,
                    Some(Relationship::P2C) => import_peering_customer += 1,
                    None => import_peering_other += 1,
                }
            }

            for import_as in appear.filter {
                match db.get(as_num, import_as) {
                    Some(Relationship::C2P) => import_filter_provider += 1,
                    Some(Relationship::P2P) => import_filter_peer += 1,
                    Some(Relationship::P2C) => import_filter_customer += 1,
                    None => import_filter_other += 1,
                }
            }

            write_comma_num!(import_provider);
            write_comma_num!(import_peer);
            write_comma_num!(import_customer);
            write_comma_num!(import_other);
            write_comma_num!(import_both_provider);
            write_comma_num!(import_both_peer);
            write_comma_num!(import_both_customer);
            write_comma_num!(import_both_other);
            write_comma_num!(import_peering_provider);
            write_comma_num!(import_filter_provider);
            write_comma_num!(import_peering_peer);
            write_comma_num!(import_filter_peer);
            write_comma_num!(import_peering_customer);
            write_comma_num!(import_filter_customer);
            write_comma_num!(import_peering_other);
            write_comma_num!(import_filter_other);
        }

        {
            let mut export_provider = 0;
            let mut export_peer = 0;
            let mut export_customer = 0;
            let mut export_other = 0;
            let mut export_self = 0;
            let mut export_peering_provider = 0;
            let mut export_filter_provider = 0;
            let mut export_peering_peer = 0;
            let mut export_filter_peer = 0;
            let mut export_peering_customer = 0;
            let mut export_filter_customer = 0;
            let mut export_peering_other = 0;
            let mut export_filter_other = 0;
            let mut export_peering_self = 0;
            let mut export_filter_self = 0;

            let mut appear = Appear {
                as_num,
                ..Default::default()
            };
            for entry in aut_num.exports.entries_iter() {
                appear.record_entry(entry, query);
            }
            appear.clean_up();

            for export_as in appear.overall {
                if as_num == export_as {
                    export_self += 1;
                }
                match db.get(as_num, export_as) {
                    Some(Relationship::C2P) => export_provider += 1,
                    Some(Relationship::P2P) => export_peer += 1,
                    Some(Relationship::P2C) => export_customer += 1,
                    None => export_other += 1,
                }
            }

            for export_as in appear.peering {
                if as_num == export_as {
                    export_peering_self += 1;
                }
                match db.get(as_num, export_as) {
                    Some(Relationship::C2P) => export_peering_provider += 1,
                    Some(Relationship::P2P) => export_peering_peer += 1,
                    Some(Relationship::P2C) => export_peering_customer += 1,
                    None => export_peering_other += 1,
                }
            }

            for export_as in appear.filter {
                if as_num == export_as {
                    export_filter_self += 1;
                }
                match db.get(as_num, export_as) {
                    Some(Relationship::C2P) => export_filter_provider += 1,
                    Some(Relationship::P2P) => export_filter_peer += 1,
                    Some(Relationship::P2C) => export_filter_customer += 1,
                    None => export_filter_other += 1,
                }
            }

            write_comma_num!(export_provider);
            write_comma_num!(export_peer);
            write_comma_num!(export_customer);
            write_comma_num!(export_other);
            write_comma_num!(export_self);
            write_comma_num!(export_peering_provider);
            write_comma_num!(export_filter_provider);
            write_comma_num!(export_peering_peer);
            write_comma_num!(export_filter_peer);
            write_comma_num!(export_peering_customer);
            write_comma_num!(export_filter_customer);
            write_comma_num!(export_peering_other);
            write_comma_num!(export_filter_other);
            write_comma_num!(export_peering_self);
            write_comma_num!(export_filter_self);
        }
        file.write_all(b"\n")?;
    }

    file.flush()?;
    drop(file);

    Ok(())
}
