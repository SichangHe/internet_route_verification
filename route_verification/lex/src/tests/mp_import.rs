use crate::{
    action::Action::*,
    filter::Filter::*,
    mp_import::{Casts, Entry, PeeringAction},
    peering::AsExpr::*,
    *,
};

use super::*;

const MP_IMPORT_EXAMPLE: &str = r#"{
    "ipv4": {
        "unicast": [
            {
                "mp_peerings": [
                    {
                        "mp_peering": {"as_expr": "AS3344:PRNG-LONAP"},
                        "actions": {
                            "pref": "64535",
                            "community": [
                                {
                                    "method": "append",
                                    "args": [
                                        "3344:60000",
                                        "3344:60020",
                                        "3344:8330"
                                    ]
                                }
                            ]
                        }
                    }
                ],
                "mp_filter": {
                    "and": {
                        "left": {"path_attr": "ANY"},
                        "right": {"not": {"path_attr": "AS3344:fltr-filterlist"}}
                    }
                }
            }
        ]
    }
}"#;

#[test]
fn mp_import() -> Result<()> {
    let expected = expected_mp_import();
    let parsed: Versions = serde_json::from_str(MP_IMPORT_EXAMPLE)?;
    assert_eq!(parsed, expected);
    Ok(())
}

fn expected_mp_import() -> Versions {
    Versions {
        any: Casts::default(),
        ipv4: Casts {
            any: vec![],
            unicast: vec![Entry {
                mp_peerings: vec![PeeringAction {
                    mp_peering: Peering {
                        as_expr: Field("AS3344:PRNG-LONAP".into()),
                        router_expr1: None,
                        router_expr2: None,
                    },
                    actions: BTreeMap::from([
                        (
                            "community".into(),
                            MethodCall(vec![Call {
                                method: Some("append".into()),
                                args: vec![
                                    "3344:60000".into(),
                                    "3344:60020".into(),
                                    "3344:8330".into(),
                                ],
                            }]),
                        ),
                        ("pref".into(), Assigned("64535".into())),
                    ]),
                }],
                mp_filter: And {
                    left: Box::new(PathAttr("ANY".into())),
                    right: Box::new(Not(Box::new(PathAttr("AS3344:fltr-filterlist".into())))),
                },
            }],
            multicast: vec![],
        },
        ipv6: Casts::default(),
    }
}
