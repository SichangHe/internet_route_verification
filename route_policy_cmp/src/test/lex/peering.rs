use std::iter::zip;

use crate::lex::peering::{AsExpr::*, ComplexAsExpr::*, Peering};

use super::*;

const PEERING_EXAMPLES: &[&str] = &[
    r#"{"as_expr": "AS51468"}"#,
    r#"{
        "as_expr": "AS8717",
        "router_expr1": "2001:67c:20d0:fffe:ffff:ffff:ffff:fffe",
        "router_expr2": "2001:67c:20d0:fffe:ffff:ffff:ffff:fffd"
    }"#,
    r#"{"as_expr": {"and": {"left": "AS9186:AS-CUSTOMERS", "right": "AS204094"}}}"#,
    r#"{"as_expr": {"except": {"left": "AS-ANY", "right": "AS5398:AS-AMS-IX-FILTER"}}}"#,
    r#"{"as_expr": {"group": {"or": {"left": "AS42", "right": "AS3856"}}}}"#,
    r#"{
        "as_expr": {
            "except": {
                "left": "AS-ANY",
                "right": {
                    "group": {
                        "or": {
                            "left": "AS40027",
                            "right": {"or": {"left": "AS63293", "right": "AS65535"}}
                        }
                    }
                }
            }
        }
    }"#,
];

#[test]
fn peering() -> Result<()> {
    let parsed_peerings = [
        Peering {
            as_expr: Field("AS51468".into()),
            router_expr1: None,
            router_expr2: None,
        },
        Peering {
            as_expr: Field("AS8717".into()),
            router_expr1: Some(Field("2001:67c:20d0:fffe:ffff:ffff:ffff:fffe".into())),
            router_expr2: Some(Field("2001:67c:20d0:fffe:ffff:ffff:ffff:fffd".into())),
        },
        Peering {
            as_expr: AsComp(And {
                left: Box::new(Field("AS9186:AS-CUSTOMERS".into())),
                right: Box::new(Field("AS204094".into())),
            }),
            router_expr1: None,
            router_expr2: None,
        },
        Peering {
            as_expr: AsComp(Except {
                left: Box::new(Field("AS-ANY".into())),
                right: Box::new(Field("AS5398:AS-AMS-IX-FILTER".into())),
            }),
            router_expr1: None,
            router_expr2: None,
        },
        Peering {
            as_expr: AsComp(Group(Box::new(AsComp(Or {
                left: Box::new(Field("AS42".into())),
                right: Box::new(Field("AS3856".into())),
            })))),
            router_expr1: None,
            router_expr2: None,
        },
        Peering {
            as_expr: AsComp(Except {
                left: Box::new(Field("AS-ANY".into())),
                right: Box::new(AsComp(Group(Box::new(AsComp(Or {
                    left: Box::new(Field("AS40027".into())),
                    right: Box::new(AsComp(Or {
                        left: Box::new(Field("AS63293".into())),
                        right: Box::new(Field("AS65535".into())),
                    })),
                }))))),
            }),
            router_expr1: None,
            router_expr2: None,
        },
    ];

    for (&peering, expected) in zip(PEERING_EXAMPLES, parsed_peerings) {
        let result: Peering = serde_json::from_str(peering)?;
        assert_eq!(result, expected);
    }
    Ok(())
}
