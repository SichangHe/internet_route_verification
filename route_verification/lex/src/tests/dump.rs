use crate::test_util::*;

use super::*;

#[test]
fn dump() -> Result<()> {
    let lexed: Dump = serde_json::from_str(DUMP)?;
    let expected = expected_dump();
    assert_eq!(lexed, expected);
    Ok(())
}
