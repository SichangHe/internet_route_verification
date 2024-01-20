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
    let (members0, set0) = graph.add_member(
        vec![
            ASNumOrSet::Num(113),
            ASNumOrSet::Num(114),
            ASNumOrSet::Num(115),
            ASNumOrSet::set("AS-DE"),
            ASNumOrSet::set("m#AS-BC"),
        ],
        ASNumOrSet::set("AS-BC"),
    );
    let (members1, set1) = graph.add_member(vec![ASNumOrSet::Num(113)], ASNumOrSet::set("AS-DE"));

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
