use anyhow::Result;
use regex_syntax::{
    hir::{
        Hir,
        Look::{End, Start},
    },
    Parser,
};

#[test]
fn pass() -> Result<()> {
    let actual = Parser::new().parse("^a b c$")?;
    let expected = Hir::concat(vec![
        Hir::look(Start),
        Hir::literal(*b"a b c"),
        Hir::look(End),
    ]);
    assert_eq!(actual, expected);
    Ok(())
}
