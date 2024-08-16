use super::*;

/// For ASes with many relaxed filter, find those simple rules and `tech-c`.
/// Copy this after running code from [`parse_bgp_lines`].
fn find_relaxed_filter_rules_n_tech_c(query: QueryIr) {
    // Read the CSVs.
    #[derive(Debug, Default)]
    struct Feature {
        export_customers: bool,
        import_customer: bool,
    }
    let mut features = HashMap::<u32, Feature>::new();
    for (path, is_export) in [
        ("scripts/many_spec_export_customers0.csv", true),
        ("scripts/many_spec_import_customer0.csv", false),
    ] {
        let bytes = std::fs::read(path).unwrap();
        let string = String::from_utf8(bytes).unwrap();
        for line in string.lines() {
            let asn = line.parse::<u32>().unwrap();
            let feature_entry = features.entry(asn).or_default();
            if is_export {
                feature_entry.export_customers = true;
            } else {
                feature_entry.import_customer = true;
            }
        }
    }

    // Get the rules, tech-c, source.
    #[derive(Debug, Default, Serialize)]
    struct AsInfo {
        export_peer_asns: Vec<u32>,
        export_as_any: bool,
        import_customer_asns: Vec<u32>,
        tech_c: String,
        source: String,
    }

    let mut as_infos = HashMap::<u32, AsInfo>::new();
    for (
        asn,
        Feature {
            export_customers,
            import_customer,
        },
    ) in features.iter()
    {
        let info_entry = as_infos.entry(*asn).or_default();
        let aut_num = query.aut_nums.get(asn).unwrap();
        let rpsl = &aut_num.body;
        for RpslExpr { key, expr } in expressions(lines_continued(rpsl.lines())) {
            match key.as_str() {
                "tech-c" => info_entry.tech_c = expr,
                "source" => info_entry.source = expr,
                _ => {}
            }
        }

        if *export_customers {
            for entry in aut_num.exports.entries_iter() {
                match (
                    entry.mp_peerings.len(),
                    entry.mp_peerings.first(),
                    &entry.mp_filter,
                ) {
                    // to <peer> announce <asn>.
                    (
                        1,
                        Some(PeeringAction {
                            mp_peering:
                                Peering {
                                    remote_as: AsExpr::Single(AsName::Num(peer)),
                                    ..
                                },
                            ..
                        }),
                        Filter::AsNum(filter_asn, _),
                    ) if *filter_asn == *asn => {
                        info_entry.export_peer_asns.push(*peer);
                    }
                    // to AS-ANY announce <asn>.
                    (
                        1,
                        Some(PeeringAction {
                            mp_peering:
                                Peering {
                                    remote_as: AsExpr::Single(AsName::Any),
                                    ..
                                },
                            ..
                        }),
                        Filter::AsNum(filter_asn, _),
                    ) if *filter_asn == *asn => {
                        info_entry.export_as_any = true;
                    }
                    _ => {}
                }
            }
        }

        if *import_customer {
            for entry in aut_num.imports.entries_iter() {
                match (
                    entry.mp_peerings.len(),
                    entry.mp_peerings.first(),
                    &entry.mp_filter,
                ) {
                    // from <customer> accept <customer>.
                    (
                        1,
                        Some(PeeringAction {
                            mp_peering:
                                Peering {
                                    remote_as: AsExpr::Single(AsName::Num(customer)),
                                    ..
                                },
                            ..
                        }),
                        Filter::AsNum(filter_asn, _),
                    ) if *filter_asn == *customer => {
                        info_entry.import_customer_asns.push(*customer);
                    }
                    _ => {}
                }
            }
        }
    }
}
