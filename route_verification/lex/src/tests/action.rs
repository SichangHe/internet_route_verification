use crate::{
    action::{
        Action::{self, *},
        Actions,
    },
    *,
};

use super::*;

const ACTION_EXAMPLE: &str = r#"{
    "pref": "65435",
    "med": "0",
    "community": [
        {"method": "append", "args": ["8226:1102"]}
    ]
}"#;

#[test]
fn action() -> Result<()> {
    let parsed: Actions = serde_json::from_str(ACTION_EXAMPLE)?;
    let expected: Actions = expected_action();
    assert_eq!(parsed, expected);
    Ok(())
}

fn expected_action() -> BTreeMap<String, Action> {
    BTreeMap::from([
        ("pref".into(), Assigned("65435".into())),
        ("med".into(), Assigned("0".into())),
        (
            "community".into(),
            MethodCall(vec![Call {
                method: Some("append".into()),
                args: vec!["8226:1102".into()],
            }]),
        ),
    ])
}
