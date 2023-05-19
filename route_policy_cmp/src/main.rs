use anyhow::Result;
use route_policy_cmp::lex::dump::Dump;
use std::fs::File;

fn main() -> Result<()> {
    env_logger::init();
    // Test lex dumped.
    let file = File::open("../dump.json")?;
    let lexed: Dump = serde_json::from_reader(file)?;
    for index in 0..10 {
        if let Some(aut_num) = lexed.aut_nums.get(index) {
            println!("aut_num: {aut_num:#?}");
        }
        if let Some(as_set) = lexed.as_sets.get(index) {
            println!("as_set: {as_set:#?}");
        }
        if let Some(route_set) = lexed.route_sets.get(index) {
            println!("route_set: {route_set:#?}");
        }
    }
    Ok(())
}
