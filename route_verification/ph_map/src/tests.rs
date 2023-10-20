use super::PerfHashMap;

#[test]
fn test_deterministic() {
    let keys: Vec<_> = ["Alex", "Bob", "Corn", "Xe"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let values = vec![532, 64, 7, 0];
    let map = PerfHashMap::new(keys.clone(), values.clone());
    for (key, value) in keys.iter().zip(values.iter()) {
        assert_eq!(map.get(key), Some(value));
    }
    let fake_keys: Vec<_> = ["Alexa", "Bobby", "Corndog", "Xerox"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    for key in fake_keys.iter() {
        assert!(map.get(key).is_none())
    }
}
