use super::*;

/// Generate statistics for routes.
/// Copy this after running code from [`parse_bgp_lines`].
fn gen_route_stats(query: QueryIr, mut bgp_lines: Vec<Line>, db: AsRelDb) -> Result<()> {
    let start = Instant::now();
    let stats: Vec<RouteStats> = bgp_lines
        .par_iter_mut()
        .map(|line| line.compare.route_stats(&query, &db))
        .collect();
    let size = stats.len();
    println!(
        "Generated stats of {size} routes in {}ms.",
        start.elapsed().as_millis()
    );

    let mut file = BufWriter::new(File::create("route_stats.csv")?);
    let header: String = "
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
            skip_regex_tilde,
            skip_regex_with_set,
            skip_community,
            unrec_import_empty,
            unrec_export_empty,
            unrec_filter_set,
            unrec_as_routes,
            unrec_route_set,
            unrec_as_set,
            unrec_as_set_route,
            unrec_some_as_set_route,
            unrec_aut_num,
            unrec_peering_set,
            spec_uphill,
            spec_uphill_tier1,
            spec_tier1_pair,
            spec_import_peer_oifps,
            spec_import_customer_oifps,
            spec_export_customers,
            spec_import_from_neighbor,
            spec_as_is_origin_but_no_route,
            spec_as_set_contains_origin_but_no_route,
            err_filter,
            err_filter_as_num,
            err_filter_as_set,
            err_filter_prefixes,
            err_filter_route_set,
            err_remote_as_num,
            err_remote_as_set,
            err_except_peering_right,
            err_peering,
            err_regex,
            rpsl_as_name,
            rpsl_filter,
            rpsl_regex,
            rpsl_unknown_filter,
            recursion,
"
    .split_ascii_whitespace()
    .collect();
    file.write_all(header.trim_end_matches(",").as_bytes());
    file.write_all(b"\n");
    let comma = b","[0];
    for s in stats {
        let RouteStats {
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
            skip_regex_tilde,
            skip_regex_with_set,
            skip_community,
            unrec_import_empty,
            unrec_export_empty,
            unrec_filter_set,
            unrec_as_routes,
            unrec_route_set,
            unrec_as_set,
            unrec_as_set_route,
            unrec_some_as_set_route,
            unrec_aut_num,
            unrec_peering_set,
            spec_uphill,
            spec_uphill_tier1,
            spec_tier1_pair,
            spec_import_peer_oifps,
            spec_import_customer_oifps,
            spec_export_customers,
            spec_import_from_neighbor,
            spec_as_is_origin_but_no_route,
            spec_as_set_contains_origin_but_no_route,
            err_filter,
            err_filter_as_num,
            err_filter_as_set,
            err_filter_prefixes,
            err_filter_route_set,
            err_remote_as_num,
            err_remote_as_set,
            err_except_peering_right,
            err_peering,
            err_regex,
            rpsl_as_name,
            rpsl_filter,
            rpsl_regex,
            rpsl_unknown_filter,
            recursion,
        } = s;
        let line: Vec<u8> = [
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
            skip_regex_tilde,
            skip_regex_with_set,
            skip_community,
            unrec_import_empty,
            unrec_export_empty,
            unrec_filter_set,
            unrec_as_routes,
            unrec_route_set,
            unrec_as_set,
            unrec_as_set_route,
            unrec_some_as_set_route,
            unrec_aut_num,
            unrec_peering_set,
            spec_uphill,
            spec_uphill_tier1,
            spec_tier1_pair,
            spec_import_peer_oifps,
            spec_import_customer_oifps,
            spec_export_customers,
            spec_import_from_neighbor,
            spec_as_is_origin_but_no_route,
            spec_as_set_contains_origin_but_no_route,
            err_filter,
            err_filter_as_num,
            err_filter_as_set,
            err_filter_prefixes,
            err_filter_route_set,
            err_remote_as_num,
            err_remote_as_set,
            err_except_peering_right,
            err_peering,
            err_regex,
            rpsl_as_name,
            rpsl_filter,
            rpsl_regex,
            rpsl_unknown_filter,
            recursion,
        ]
        .map(|b| b.to_string().into_bytes())
        .join(&comma);
        file.write_all(&line);
        file.write_all(b"\n");
    }
    file.flush()?;
    drop(file);

    Ok(())
}
