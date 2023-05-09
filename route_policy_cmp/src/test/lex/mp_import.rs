use std::collections::BTreeMap;

use crate::lex::{
    action::Action::*,
    community::Call,
    filter::{Base::*, Filter::*, Policy::*},
    mp_import::{Casts, Entry, PeeringAction, Versions},
    peering::{AsExpr::*, Peering},
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
                        "left": ["ANY"],
                        "right": {"not": ["AS3344:fltr-filterlist"]}
                    }
                }
            }
        ]
    }
}"#;

#[test]
fn mp_import() -> Result<()> {
    let expected = Versions {
        any: None,
        ipv4: Some(Casts {
            any: None,
            unicast: Some(vec![Entry {
                mp_peerings: vec![PeeringAction {
                    mp_peering: Peering {
                        as_expr: Field("AS3344:PRNG-LONAP".into()),
                        router_expr1: None,
                        router_expr2: None,
                    },
                    actions: Some(BTreeMap::from([
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
                    ])),
                }],
                mp_filter: Mixed(And {
                    left: Box::new(Policies(vec![PathAttr("ANY".into())])),
                    right: Box::new(Mixed(Not(Box::new(Policies(vec![PathAttr(
                        "AS3344:fltr-filterlist".into(),
                    )]))))),
                }),
            }]),
            multicast: None,
        }),
        ipv6: None,
    };
    let parsed: Versions = serde_json::from_str(MP_IMPORT_EXAMPLE)?;
    assert_eq!(parsed, expected);
    Ok(())
}
