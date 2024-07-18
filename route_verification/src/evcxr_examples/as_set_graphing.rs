use super::*;

/// Fully flatten each AS Set to all of its members.
/// Copy from the `:dep` line after running code from [`parse_bgp_lines`].
fn as_sets_graph_stats(ir: Ir) {
    use graph as route_verification_graph;
    /*
    :dep route_verification_graph = { path = "route_verification/graph" }
    // */
    use route_verification_graph::{ASNumOrSet, ASSetGraph, ASSetGraphStats};
    fn extend(
        as_set_graph: &mut ASSetGraph,
        as_num_or_set: ASNumOrSet,
        as_set: &AsSet,
        as_sets: &std::collections::BTreeMap<String, AsSet>,
    ) {
        let set_members_cloned: Vec<ASNumOrSet> = as_set
            .set_members
            .iter()
            .cloned()
            .map(ASNumOrSet::Set)
            .collect();
        let unseen_sets: Vec<ASNumOrSet> = set_members_cloned
            .iter()
            .filter(|set| !as_set_graph.as_num_and_sets.contains_key(*set))
            .cloned()
            .collect();

        as_set_graph.add_members(
            itertools::chain(
                set_members_cloned,
                as_set.members.iter().copied().map(ASNumOrSet::Num),
            ),
            as_num_or_set,
        );

        for unseen_set in unseen_sets {
            if let ASNumOrSet::Set(set_name) = &unseen_set {
                if let Some(as_set) = as_sets.get(set_name) {
                    extend(as_set_graph, unseen_set, as_set, as_sets);
                }
            }
        }
    }

    let start = Instant::now();
    let as_set_graph_stats: HashMap<String, (ASSetGraphStats, bool)> = ir
        .as_sets
        .par_iter()
        .map(|(name, set)| {
            let as_num_or_set = ASNumOrSet::Set(name.into());
            let mut as_set_graph =
                ASSetGraph::with_capacity(set.set_members.len() * 32 + set.members.len());
            let node_index = as_set_graph.get_or_insert(as_num_or_set.clone());
            extend(&mut as_set_graph, as_num_or_set, set, &ir.as_sets);

            let stats = as_set_graph.count_stats(node_index);
            let has_cycle = as_set_graph.has_cycle();

            (name.to_owned(), (stats, has_cycle))
        })
        .collect();
    println!(
        "Computed graphs for {} AS Sets in {}ms.",
        as_set_graph_stats.len(),
        start.elapsed().as_millis()
    );

    {
        let mut as_set_graph_stats_file =
            BufWriter::new(File::create("as_set_graph_stats.csv").unwrap());
        as_set_graph_stats_file
            .write_all(b"as_set,n_sets,n_nums,depth,has_cycle\n")
            .unwrap();
        for (
            set,
            (
                ASSetGraphStats {
                    n_sets,
                    n_nums,
                    depth,
                },
                has_cycle,
            ),
        ) in &as_set_graph_stats
        {
            as_set_graph_stats_file.write_all(set.as_bytes());
            as_set_graph_stats_file.write_all(b",");
            as_set_graph_stats_file.write_all(n_sets.to_string().as_bytes());
            as_set_graph_stats_file.write_all(b",");
            as_set_graph_stats_file.write_all(n_nums.to_string().as_bytes());
            as_set_graph_stats_file.write_all(b",");
            as_set_graph_stats_file.write_all(depth.to_string().as_bytes());
            as_set_graph_stats_file.write_all(b",");
            as_set_graph_stats_file.write_all(has_cycle.to_string().as_bytes());
            as_set_graph_stats_file.write_all(b"\n");
        }
        as_set_graph_stats_file.flush().unwrap();
    }
}
