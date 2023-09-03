use super::*;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Peering {
    pub as_expr: AsExpr,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_expr1: Option<AsExpr>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_expr2: Option<AsExpr>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum AsExpr {
    Field(String),
    AsComp(ComplexAsExpr),
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ComplexAsExpr {
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
