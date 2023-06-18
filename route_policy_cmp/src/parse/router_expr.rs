use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use crate::lex::peering::{self, ComplexAsExpr};

pub fn parse_router_expr(router_expr: peering::AsExpr) -> RouterExpr {
    match router_expr {
        peering::AsExpr::Field(field) => parse_simple_router_expr(field),
        peering::AsExpr::AsComp(comp) => parse_complex_router_expr(comp),
    }
}

pub fn parse_simple_router_expr(field: String) -> RouterExpr {
    if let Ok(ip) = field.parse() {
        RouterExpr::Ip(ip)
    } else {
        RouterExpr::InetRtrOrRtrSet(field)
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

/// Expressions over router IP addresses, inet-rtr names, and rtr-set names
/// using operators AND, OR, and EXCEPT.
/// <https://www.rfc-editor.org/rfc/rfc2622#page-25>
///
/// Currently, we don't have a means to check router expressions.
/// <https://github.com/SichangHe/parse_rpsl_policy/issues/13>.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RouterExpr {
    Ip(IpAddr),
    // Enhancement: distinguish between inet-rtr and rtr-set.
    InetRtrOrRtrSet(String),
    And {
        left: Box<RouterExpr>,
        right: Box<RouterExpr>,
    },
    Or {
        left: Box<RouterExpr>,
        right: Box<RouterExpr>,
    },
    Except {
        left: Box<RouterExpr>,
        right: Box<RouterExpr>,
    },
    Group(Box<RouterExpr>),
}
