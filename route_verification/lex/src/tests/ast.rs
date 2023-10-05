use crate::test_util::*;

use super::*;

#[test]
fn ast() -> Result<()> {
    let lexed: Ast = serde_json::from_str(AST)?;
    let expected = expected_ast();
    assert_eq!(lexed, expected);
    Ok(())
}
