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
    for (s, expected, char_map) in AS_REGEXES {
        let mut replacer = CharMap::new_from_alpha();
        let replaced = as_replace_all(s, replacer.by_ref());
        assert_eq!(&replaced, expected);
        assert_eq!(replacer.next, char_map.len() as u32 + ALPHA_CODE);
        assert_eq!(&replacer.char_map, char_map);
    }
    Ok(())
}

const AS_REGEXES: [(&str, &str, &[&str]); 3] = [
    ("^AS20485 AS15774$", "^Α Β$", &["AS20485", "AS15774"]),
    ("^AS611+AS6509.*$", "^Α+Β.*$", &["AS611", "AS6509"]),
    (
        "^AS24167.*(AS1659|AS9916)?$",
        "^Α.*(Β|Γ)?$",
        &["AS24167", "AS1659", "AS9916"],
    ),
];

#[test]
fn replace_as_set() -> Result<()> {
    for (s, expected, char_map) in AS_SET_REGEXES {
        let mut replacer = CharMap::new_from_alpha();
        let replaced = as_set_replace_all(s, replacer.by_ref());
        assert_eq!(&replaced, expected);
        assert_eq!(replacer.next, char_map.len() as u32 + ALPHA_CODE);
        assert_eq!(&replacer.char_map, char_map);
    }
    Ok(())
}

const AS_SET_REGEXES: [(&str, &str, &[&str]); 3] = [
    ("^AS60725:AS-O3B-HI-US+$", "^Α+$", &["AS60725:AS-O3B-HI-US"]),
    (
        "^AS22573+AS22573:AS-CUSTOMERS*$",
        "^AS22573+Α*$",
        &["AS22573:AS-CUSTOMERS"],
    ),
    (
        "^AS38611+ AS2764:AS-TRANSIT:AS38611+ AS2764:AS-CUSTOMERS:AS38611~*$",
        "^AS38611+ Α+ Β~*$",
        &["AS2764:AS-TRANSIT:AS38611", "AS2764:AS-CUSTOMERS:AS38611"],
    ),
];
