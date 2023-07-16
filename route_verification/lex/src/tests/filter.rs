use std::iter::zip;

use super::{
    Filter::{self, *},
    *,
};

const FILTER_EXAMPLES: &[&str] = &[
    r#"{"path_attr": "AS-UNIC"}"#,
    r#"{
    "and": {
        "left": {"path_attr": "ANY"},
        "right": {"addr_prefix_set": ["0.0.0.0/0^0-24"]}
    }
}"#,
    r#"{
    "and": {
        "left": {"path_attr": "as-foo"},
        "right": {
            "and": {
                "left": {"path_attr": "AS65226"},
                "right": {"addr_prefix_set": ["2001:0DB8::/32"]}
            }
        }
    }
}"#,
];

#[test]
fn filter() -> Result<()> {
    let parsed_filters = expected_filters();
    for (&filter, expected) in zip(FILTER_EXAMPLES, parsed_filters) {
        let parsed: Filter = serde_json::from_str(filter)?;
        assert_eq!(parsed, expected);
    }
    Ok(())
}

fn expected_filters() -> [Filter; 3] {
    [
        PathAttr("AS-UNIC".into()),
        And {
            left: Box::new(PathAttr("ANY".into())),
            right: Box::new(AddrPrefixSet(vec!["0.0.0.0/0^0-24".into()])),
        },
        And {
            left: Box::new(PathAttr("as-foo".into())),
            right: Box::new(And {
                left: Box::new(PathAttr("AS65226".into())),
                right: Box::new(AddrPrefixSet(vec!["2001:0DB8::/32".into()])),
            }),
        },
    ]
}
