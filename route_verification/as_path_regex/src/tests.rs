use super::*;

#[test]
fn replace_as() -> Result<()> {
    for (s, expected, char_map) in AS_REGEXES {
        let mut replacer = CharMap::<u64>::new_from_alpha();
        let replaced = as_replace_all(s, replacer.by_ref());
        assert_eq!(&replaced, expected);
        assert_eq!(replacer.next, char_map.len() as u32 + ALPHA_CODE);
        assert_eq!(&replacer.char_map, char_map);
    }
    Ok(())
}

const AS_REGEXES: [(&str, &str, &[u64]); 3] = [
    ("^AS20485 AS15774$", "^Α Β$", &[20485, 15774]),
    ("^AS611+AS6509.*$", "^Α+Β.*$", &[611, 6509]),
    (
        "^AS24167.*(AS1659|AS9916)?$",
        "^Α.*(Β|Γ)?$",
        &[24167, 1659, 9916],
    ),
];

#[test]
fn replace_as_set() -> Result<()> {
    for (s, expected, char_map) in AS_SET_REGEXES {
        let mut replacer = CharMap::<String>::new_from_alpha();
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

#[test]
fn interpret_as() -> Result<()> {
    for ((s, _, _), expected) in AS_REGEXES.into_iter().zip(EXPECTED_SIMPLE_EVENTS_DEBUG) {
        let interpreter: Interpreter = s.parse()?;
        let events = interpreter.into_iter().collect::<Result<Vec<_>, _>>()?;
        let events_debug = format!("{events:?}");
        assert_eq!(events_debug, expected);
    }
    Ok(())
}

const EXPECTED_SIMPLE_EVENTS_DEBUG: [&str; 3] = [
    "[Start, Literal(AsNum(20485)), Literal(AsNum(15774)), End]",
    "[Start, Repeat { min: 1, max: None, greedy: true, walker: Walker { init_state: Literal(\"Α\"), rems: [Ir(Literal(\"Α\"))] } }, Literal(AsNum(6509)), Repeat { min: 0, max: None, greedy: true, walker: Walker { init_state: Class({'\\0'..='\\t', '\\u{b}'..='\\u{10ffff}'}), rems: [Ir(Class({'\\0'..='\\t', '\\u{b}'..='\\u{10ffff}'}))] } }, End]",
    "[Start, Literal(AsNum(24167)), Repeat { min: 0, max: None, greedy: true, walker: Walker { init_state: Class({'\\0'..='\\t', '\\u{b}'..='\\u{10ffff}'}), rems: [Ir(Class({'\\0'..='\\t', '\\u{b}'..='\\u{10ffff}'}))] } }, Repeat { min: 0, max: Some(1), greedy: true, walker: Walker { init_state: Capture(Capture { index: 1, name: None, sub: Class({'Β'..='Γ'}) }), rems: [Ir(Capture(Capture { index: 1, name: None, sub: Class({'Β'..='Γ'}) }))] } }, End]",
];
