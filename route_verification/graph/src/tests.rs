use super::*;

#[test]
fn insertions() {
    let mut graph = ASSetGraph::default();
    let node0 = graph.get_or_insert(ASNumOrSet::set("AS-BC"));
    let node1 = graph.get_or_insert(ASNumOrSet::set("AS-DE"));
    let node2 = graph.get_or_insert(ASNumOrSet::Num(113));

    let graph_str = format!("{graph}");
    print!("{graph_str}");
    let expected_str = r#"digraph {
    0 [ label = "AS-BC" ]
    1 [ label = "AS-DE" ]
    2 [ label = "AS113" ]
}
"#;
    assert_eq!(&graph_str, expected_str);

    assert_eq!(node0.index(), 0);
    assert_eq!(node1.index(), 1);
    assert_eq!(node2.index(), 2);
}

#[test]
fn add_members() {
    let mut graph = ASSetGraph::default();
    let (members0, set0) = graph.add_members(
        vec![
            ASNumOrSet::Num(113),
            ASNumOrSet::Num(114),
            ASNumOrSet::Num(115),
            ASNumOrSet::set("AS-DE"),
            ASNumOrSet::set("m#AS-BC"),
        ],
        ASNumOrSet::set("AS-BC"),
    );
    let (members1, set1) = graph.add_members(vec![ASNumOrSet::Num(113)], ASNumOrSet::set("AS-DE"));

    let graph_str = format!("{graph}");
    print!("{graph_str}");
    let expected_str = r#"digraph {
    0 [ label = "AS-BC" ]
    1 [ label = "AS113" ]
    2 [ label = "AS114" ]
    3 [ label = "AS115" ]
    4 [ label = "AS-DE" ]
    5 [ label = "m#AS-BC" ]
    0 -> 1 [ label = "1" ]
    0 -> 2 [ label = "1" ]
    0 -> 3 [ label = "1" ]
    0 -> 4 [ label = "1" ]
    0 -> 5 [ label = "0" ]
    4 -> 1 [ label = "1" ]
}
"#;
    assert_eq!(&graph_str, expected_str);

    assert_eq!(set0.index(), 0);
    let members0_indexes: Vec<_> = members0.into_iter().map(|n| n.index()).collect();
    assert_eq!(members0_indexes, vec![1, 2, 3, 4, 5]);

    assert_eq!(set1.index(), 4);
    let members1_indexes: Vec<_> = members1.into_iter().map(|n| n.index()).collect();
    assert_eq!(members1_indexes, vec![1]);
}

#[test]
fn stats() {
    let mut graph = ASSetGraph::default();
    let (_, set_index) = graph.add_members(
        vec![
            ASNumOrSet::Num(113),
            ASNumOrSet::Num(114),
            ASNumOrSet::Num(115),
            ASNumOrSet::set("AS-DE"),
            ASNumOrSet::set("AS-FG"),
            ASNumOrSet::set("AS-HI"),
            ASNumOrSet::set("m#AS-BC"),
        ],
        ASNumOrSet::set("AS-BC"),
    );
    graph.add_members(
        vec![
            ASNumOrSet::Num(113),
            ASNumOrSet::set("AS-BC"),
            ASNumOrSet::set("AS-FG"),
        ],
        ASNumOrSet::set("AS-DE"),
    );
    graph.add_members(
        vec![
            ASNumOrSet::Num(579),
            ASNumOrSet::set("AS-DE"),
            ASNumOrSet::set("AS-HI"),
            ASNumOrSet::set("AS-JK"),
        ],
        ASNumOrSet::set("AS-FG"),
    );
    graph.add_members(
        vec![
            ASNumOrSet::Num(579),
            ASNumOrSet::Num(24),
            ASNumOrSet::Num(108),
        ],
        ASNumOrSet::set("AS-HI"),
    );
    graph.add_members(vec![ASNumOrSet::Num(40)], ASNumOrSet::set("AS-JK"));

    let graph_str = format!("{graph}");
    print!("{graph_str}");
    let expected_str = r#"digraph {
    0 [ label = "AS-BC" ]
    1 [ label = "AS113" ]
    2 [ label = "AS114" ]
    3 [ label = "AS115" ]
    4 [ label = "AS-DE" ]
    5 [ label = "AS-FG" ]
    6 [ label = "AS-HI" ]
    7 [ label = "m#AS-BC" ]
    8 [ label = "AS579" ]
    9 [ label = "AS-JK" ]
    10 [ label = "AS24" ]
    11 [ label = "AS108" ]
    12 [ label = "AS40" ]
    0 -> 1 [ label = "1" ]
    0 -> 2 [ label = "1" ]
    0 -> 3 [ label = "1" ]
    0 -> 4 [ label = "1" ]
    0 -> 5 [ label = "1" ]
    0 -> 6 [ label = "1" ]
    0 -> 7 [ label = "0" ]
    4 -> 1 [ label = "1" ]
    4 -> 0 [ label = "1" ]
    4 -> 5 [ label = "1" ]
    5 -> 8 [ label = "1" ]
    5 -> 4 [ label = "1" ]
    5 -> 6 [ label = "1" ]
    5 -> 9 [ label = "1" ]
    6 -> 8 [ label = "1" ]
    6 -> 10 [ label = "1" ]
    6 -> 11 [ label = "1" ]
    9 -> 12 [ label = "1" ]
}
"#;
    assert_eq!(graph_str, expected_str);

    let stats = graph.count_stats(set_index);
    let expected_stats = ASSetGraphStats {
        n_sets: 4,
        n_nums: 7,
        depth: 3,
    };
    assert_eq!(stats, expected_stats);

    let has_cycle = graph.has_cycle();
    assert!(has_cycle);
}
