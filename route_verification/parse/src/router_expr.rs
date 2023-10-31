use ::lex::{self, ComplexAsExpr};

use super::*;

pub fn parse_router_expr(router_expr: lex::AsExpr) -> RouterExpr {
    match router_expr {
        lex::AsExpr::Field(field) => parse_simple_router_expr(field),
        lex::AsExpr::AsComp(comp) => parse_complex_router_expr(comp),
    }
}

pub fn parse_complex_router_expr(router_expr: ComplexAsExpr) -> RouterExpr {
    use RouterExpr::*;
    match router_expr {
        ComplexAsExpr::And { left, right } => And {
            left: Box::new(parse_router_expr(*left)),
            right: Box::new(parse_router_expr(*right)),
        },
        ComplexAsExpr::Or { left, right } => Or {
            left: Box::new(parse_router_expr(*left)),
            right: Box::new(parse_router_expr(*right)),
        },
        ComplexAsExpr::Except { left, right } => Except {
            left: Box::new(parse_router_expr(*left)),
            right: Box::new(parse_router_expr(*right)),
        },
        ComplexAsExpr::Group(group) => Group(Box::new(parse_router_expr(*group))),
    }
}
