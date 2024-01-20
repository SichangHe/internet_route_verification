use super::*;

#[test]
fn insertions() {
    let mut graph = ASSetGraph::default();
    let node0 = graph.get_or_insert(ASNumOrSet::set("AS-BC"));
    let node1 = graph.get_or_insert(ASNumOrSet::set("AS-DE"));
    let node2 = graph.get_or_insert(ASNumOrSet::Num(113));
    println!("{graph}");
    assert!(node0.index() == 0);
    assert!(node1.index() == 1);
    assert!(node2.index() == 2);
}
