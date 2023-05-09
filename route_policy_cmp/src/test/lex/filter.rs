use std::iter::zip;

use crate::lex::filter::{
    Base::*,
    Filter::{self, *},
    Policy::*,
};

use super::*;

const FILTER_EXAMPLES: &[&str] = &[
    r#"["AS-UNIC"]"#,
    r#"{
    "and": {"left": ["ANY"], "right": [["0.0.0.0/0^0-24"]]}
}"#,
    r#"{
    "and": {
        "left": ["as-foo"],
        "right": {
            "and": {
                "left": ["AS65226"],
                "right": [["2001:0DB8::/32"]]
            }
        }
    }
}"#,
];

#[test]
fn filter() -> Result<()> {
    let parsed_filters = [
        Policies(vec![PathAttr("AS-UNIC".into())]),
        Mixed(And {
            left: Box::new(Policies(vec![PathAttr("ANY".into())])),
            right: Box::new(Policies(vec![AddrPrefixSet(vec!["0.0.0.0/0^0-24".into()])])),
        }),
        Mixed(And {
            left: Box::new(Policies(vec![PathAttr("as-foo".into())])),
            right: Box::new(Mixed(And {
                left: Box::new(Policies(vec![PathAttr("AS65226".into())])),
                right: Box::new(Policies(vec![AddrPrefixSet(vec!["2001:0DB8::/32".into()])])),
            })),
        }),
    ];
    for (&filter, expected) in zip(FILTER_EXAMPLES, parsed_filters) {
        let parsed: Filter = serde_json::from_str(filter)?;
        assert_eq!(parsed, expected);
    }
    Ok(())
}
