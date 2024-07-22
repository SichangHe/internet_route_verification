use super::*;

/// Generate statistics for AS neighbors vs rules.
/// Copy the content after running code from [`parse_bgp_lines`].
fn as_w_routes_wo_aut_num(query: QueryIr) {
    let as_w_routes_wo_aut_num: Vec<_> = query
        .as_routes
        .par_iter()
        .filter(|(num, _)| !query.aut_nums.contains_key(*num))
        .map(|(num, routes)| (*num, routes.len()))
        .collect();

    {
        let mut file = BufWriter::new(File::create("as_w_routes_wo_aut_num.csv").unwrap());
        file.write_all(b"as_num,n_route\n").unwrap();
        for (as_num, n_route) in &as_w_routes_wo_aut_num {
            file.write_all(format!("{},{}\n", as_num, n_route).as_bytes())
                .unwrap();
        }
        file.flush().unwrap();
    }
}
