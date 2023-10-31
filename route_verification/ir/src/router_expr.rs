use std::net::IpAddr;

use super::*;

pub fn parse_simple_router_expr(field: String) -> RouterExpr {
    if let Ok(ip) = field.parse() {
        RouterExpr::Ip(ip)
    } else {
        RouterExpr::InetRtrOrRtrSet(field)
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
