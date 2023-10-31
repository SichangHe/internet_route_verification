use ::lex::{mp_import, peering};

use super::*;

pub fn parse_mp_peerings(mp_peerings: Vec<mp_import::PeeringAction>) -> Vec<PeeringAction> {
    mp_peerings.into_iter().map(parse_peering_action).collect()
}

pub fn parse_peering_action(peering_action: mp_import::PeeringAction) -> PeeringAction {
    let mp_import::PeeringAction {
        mp_peering,
        actions,
    } = peering_action;
    let mp_peering = parse_mp_peering(mp_peering);
    let actions = parse_actions(actions);
    PeeringAction {
        mp_peering,
        actions,
    }
}

pub fn parse_mp_peering(mp_peering: peering::Peering) -> Peering {
    let peering::Peering {
        as_expr,
        router_expr1,
        router_expr2,
    } = mp_peering;
    let remote_as = parse_as_expr(as_expr);
    let remote_router = router_expr1.map(parse_router_expr);
    let local_router = router_expr2.map(parse_router_expr);
    Peering {
        remote_as,
        remote_router,
        local_router,
    }
}

pub fn parse_as_expr(as_expr: peering::AsExpr) -> AsExpr {
    match as_expr {
        peering::AsExpr::Field(single) => parse_single_as_expr(single),
        peering::AsExpr::AsComp(comp) => parse_complex_as_expr(comp),
    }
}

pub fn parse_complex_as_expr(comp: peering::ComplexAsExpr) -> AsExpr {
    use AsExpr::*;
    match comp {
        peering::ComplexAsExpr::And { left, right } => And {
            left: Box::new(parse_as_expr(*left)),
            right: Box::new(parse_as_expr(*right)),
        },
        peering::ComplexAsExpr::Or { left, right } => Or {
            left: Box::new(parse_as_expr(*left)),
            right: Box::new(parse_as_expr(*right)),
        },
        peering::ComplexAsExpr::Except { left, right } => Except {
            left: Box::new(parse_as_expr(*left)),
            right: Box::new(parse_as_expr(*right)),
        },
        peering::ComplexAsExpr::Group(group) => Group(Box::new(parse_as_expr(*group))),
    }
}
