use super::BloomHashSet;

#[test]
fn test_insert() {
    let mut m = BloomHashSet::with_capacity(8, 16);
    assert_eq!(m.len(), 0);
    assert!(!m.contains(&1));
    m.insert(1);
    assert_eq!(m.len(), 1);
    assert!(!m.contains(&2));
    m.insert(2);
    assert_eq!(m.len(), 2);
    assert!(m.contains(&1));
    assert!(m.contains(&2));
}
