use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Peering {
    pub as_expr: AsExpr,
    pub router_expr1: Option<AsExpr>,
    pub router_expr2: Option<AsExpr>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AsExpr {
    Field(String),
    Complex(ComplexExpr),
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ComplexExpr {
    And {
        left: Box<AsExpr>,
        right: Box<AsExpr>,
    },
    Or {
        left: Box<AsExpr>,
        right: Box<AsExpr>,
    },
    Except {
        left: Box<AsExpr>,
        right: Box<AsExpr>,
    },
    Group(Box<AsExpr>),
}
