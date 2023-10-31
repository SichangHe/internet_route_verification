use ::lex;

use super::*;

pub fn parse_mp_peerings(mp_peerings: Vec<lex::PeeringAction>) -> Vec<PeeringAction> {
    mp_peerings.into_iter().map(parse_peering_action).collect()
}

pub fn parse_peering_action(peering_action: lex::PeeringAction) -> PeeringAction {
    let lex::PeeringAction {
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

pub fn parse_mp_peering(mp_peering: lex::Peering) -> Peering {
    let lex::Peering {
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

pub fn parse_as_expr(as_expr: lex::AsExpr) -> AsExpr {
    match as_expr {
        lex::AsExpr::Field(single) => parse_single_as_expr(single),
        lex::AsExpr::AsComp(comp) => parse_complex_as_expr(comp),
    }
}

pub fn parse_complex_as_expr(comp: lex::ComplexAsExpr) -> AsExpr {
    use AsExpr::*;
    match comp {
        lex::ComplexAsExpr::And { left, right } => And {
            left: Box::new(parse_as_expr(*left)),
            right: Box::new(parse_as_expr(*right)),
        },
        lex::ComplexAsExpr::Or { left, right } => Or {
            left: Box::new(parse_as_expr(*left)),
            right: Box::new(parse_as_expr(*right)),
        },
        lex::ComplexAsExpr::Except { left, right } => Except {
            left: Box::new(parse_as_expr(*left)),
            right: Box::new(parse_as_expr(*right)),
        },
        lex::ComplexAsExpr::Group(group) => Group(Box::new(parse_as_expr(*group))),
    }
}
