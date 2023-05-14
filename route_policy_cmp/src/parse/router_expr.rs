use serde::{Deserialize, Serialize};

use crate::lex::peering;

pub fn parse_router_expr(router_expr: peering::AsExpr) -> RouterExpr {
    // TODO: Implement.
    RouterExpr::Todo
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RouterExpr {
    Todo, // TODO: Fill in
}
