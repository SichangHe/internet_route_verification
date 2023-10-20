use maplit::hashmap;

use super::{cmp::*, *};

#[test]
fn psedo_customer_set() -> Result<()> {
    let ir = ir()?;
    let db = as_relationship_db()?;
    let mut actual: HashMap<_, _> = QueryIr::from_ir_and_as_relationship(ir, &db)
        .as_sets
        .raw
        .into_iter()
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .collect();
    actual.iter_mut().for_each(|(_, v)| v.members.sort());
    assert_eq!(actual, expected_as_sets());
    Ok(())
}

fn expected_as_sets() -> HashMap<String, AsSet> {
    hashmap! {"c#2914".into()=> AsSet { body: "".into(), members: vec![4096, 9583], set_members: vec![], is_any: false }, "c#1239".into()=> AsSet { body: "".into(), members: vec![3130], set_members: vec![], is_any: false }}
}
