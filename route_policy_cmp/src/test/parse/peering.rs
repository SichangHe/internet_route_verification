use crate::{lex, parse::*};

#[test]
fn peering_set() {
    for name in [
        "AS8785:prng-nyiix".to_string(),
        "prng-as5408-grix-peers".to_string(),
    ] {
        let parsed = parse_mp_peering(peering_field(name.clone()));
        let expected = Peering {
            remote_as: AsExpr::PeeringSet(name),
            remote_router: None,
            local_router: None,
        };
        assert_eq!(parsed, expected);
    }

    let name = "AS5408:RS-ROUTES^0-32";
    let is_set = is_peering_set(name);
    assert!(!is_set);
}

fn peering_field(name: String) -> lex::Peering {
    lex::Peering {
        as_expr: lex::AsExpr::Field(name),
        router_expr1: None,
        router_expr2: None,
    }
}
