use anyhow::Result;
use regex_syntax::{
    hir::{
        Hir,
        Look::{End, Start},
    },
    Parser,
};

use crate::*;

#[test]
fn trivial_parser() -> Result<()> {
    let actual = Parser::new().parse("^a b c$")?;
    let expected = Hir::concat(vec![
        Hir::look(Start),
        Hir::literal(*b"a b c"),
        Hir::look(End),
    ]);
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn replace_as() -> Result<()> {
    let s = "^AS1 AS2 AS3$";
    let mut replacer = CharMap::new_from_alpha();
    let replaced = as_replace_all(s, replacer.by_ref());
    assert_eq!(&replaced, "^Α Β Γ$");
    assert_eq!(replacer.next, ALPHA_CODE + 3);
    assert_eq!(replacer.char_map, vec!["AS1", "AS2", "AS3"]);
    Ok(())
}
