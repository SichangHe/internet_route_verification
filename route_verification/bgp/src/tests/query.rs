use maplit::hashmap;

use super::{cmp::*, *};

#[test]
fn psedo_customer_set() -> Result<()> {
    let dump = dump()?;
    let db = as_relationship_db()?;
    let mut actual = HashMap::from_iter(QueryDump::from_dump_and_as_relations(dump, &db).as_sets);
    actual.iter_mut().for_each(|(_, v)| v.members.sort());
    assert_eq!(actual, expected_as_sets());
    Ok(())
}

fn expected_as_sets() -> HashMap<String, AsSet> {
    hashmap! {"c#2914".into()=> AsSet { body: "".into(), members: vec![4096, 9583], set_members: vec![] }, "c#1239".into()=> AsSet { body: "".into(), members: vec![3130], set_members: vec![] }}
}
