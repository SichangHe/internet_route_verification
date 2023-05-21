use crate::lex::peering;
use crate::parse::peering::{parse_mp_peering, try_parse_peering_set, Peering};

#[test]
fn peering_set() {
    for name in [
        "AS8785:prng-nyiix".to_string(),
        "prng-as5408-grix-peers".to_string(),
    ] {
        let parsed = parse_mp_peering(peering_field(name.clone()));
        let expected = Peering::PeeringSet(name);
        assert_eq!(parsed, expected);
    }

    let name = "AS5408:RS-ROUTES^0-32".into();
    let parsed = try_parse_peering_set(&peering_field(name));
    assert_eq!(parsed, None);
}

fn peering_field(name: String) -> peering::Peering {
    peering::Peering {
        as_expr: peering::AsExpr::Field(name),
        router_expr1: None,
        router_expr2: None,
    }
}
