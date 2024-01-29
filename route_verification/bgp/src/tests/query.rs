use hashbrown::HashSet;
use maplit::hashmap;

use super::{cmp::*, *};

#[test]
fn psedo_customer_set() -> Result<()> {
    let ir = ir()?;
    let db = as_relationship_db()?;
    let actual = HashMap::from_iter(QueryIr::from_ir_and_as_relationship(ir, &db).as_sets);
    assert_eq!(actual, expected_as_sets());
    Ok(())
}

fn expected_as_sets() -> HashMap<String, QueryAsSet> {
    hashmap! {
        "c#2914".into()=> QueryAsSet {
            body: "".into(), members:  HashSet::from([4096, 9583]), unrecorded_members: vec![], is_any: false
        },
        "c#1239".into()=> QueryAsSet {
            body: "".into(), members: HashSet::from([3130]), unrecorded_members: vec![], is_any: false
        }
    }
}
