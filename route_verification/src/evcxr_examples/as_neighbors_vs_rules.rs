use super::*;

/// Generate statistics for AS neighbors vs rules.
/// Copy the content after running code from [`parse_bgp_lines`].
fn as_neighbors_vs_rules(query: QueryIr, mut bgp_lines: Vec<Line>, db: AsRelDb) {
    struct NeighborRuleStats {
        provider: i32,
        peer: i32,
        customer: i32,
        import: i32,
        export: i32,
    }

    const fn init_neighbor_stats() -> NeighborRuleStats {
        NeighborRuleStats {
            provider: 0,
            peer: 0,
            customer: 0,
            import: -1,
            export: -1,
        }
    }

    let map: DashMap<u32, NeighborRuleStats> = DashMap::new();
    db.source2dest
        .par_iter()
        .for_each(|((as1, as2), relationship)| {
            let (provider, customer) = match relationship {
                Relationship::P2P => {
                    map.entry(*as1).or_insert(init_neighbor_stats()).peer += 1;
                    map.entry(*as2).or_insert(init_neighbor_stats()).peer += 1;
                    return;
                }
                Relationship::P2C => (as1, as2),
                Relationship::C2P => (as2, as1),
            };
            map.entry(*provider)
                .or_insert(init_neighbor_stats())
                .customer += 1;
            map.entry(*customer)
                .or_insert(init_neighbor_stats())
                .provider += 1;
        });

    const fn init_rule_stats() -> NeighborRuleStats {
        NeighborRuleStats {
            provider: -1,
            peer: -1,
            customer: -1,
            import: 0,
            export: 0,
        }
    }

    query.aut_nums.par_iter().for_each(|(num, an)| {
        let mut entry = map.entry(*num).or_insert(init_rule_stats());
        entry.import = an.imports.len() as i32;
        entry.export = an.exports.len() as i32;
    });

    let mut file = BufWriter::new(File::create("as_neighbors_vs_rules5.csv").unwrap());
    file.write_all(b"aut_num,provider,peer,customer,import,export\n")
        .unwrap();
    for (
        an,
        NeighborRuleStats {
            provider,
            peer,
            customer,
            import,
            export,
        },
    ) in map.into_iter()
    {
        file.write_all(format!("{an},{provider},{peer},{customer},{import},{export}\n").as_bytes())
            .unwrap();
    }
    file.flush().unwrap();
    drop(file);
}
