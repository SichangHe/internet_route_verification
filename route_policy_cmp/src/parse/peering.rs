use lazy_regex::regex_captures;
use serde::{Deserialize, Serialize};

use crate::lex::{mp_import, peering};

use super::{
    action::{parse_actions, Actions},
    router_expr::{parse_router_expr, RouterExpr},
};

pub fn parse_mp_peerings(mp_peerings: Vec<mp_import::PeeringAction>) -> Vec<PeeringAction> {
    mp_peerings.into_iter().map(parse_peering_action).collect()
}

pub fn parse_peering_action(peering_action: mp_import::PeeringAction) -> PeeringAction {
    let mp_import::PeeringAction {
        mp_peering,
        actions,
    } = peering_action;
    let mp_peering = parsing_mp_peering(mp_peering);
    let actions = actions.map(parse_actions);
    PeeringAction {
        mp_peering,
        actions,
    }
}

pub fn parsing_mp_peering(mp_peering: peering::Peering) -> Peering {
    if let Some(set) = try_parse_peering_set(&mp_peering) {
        // <peering-set>
        return set;
    }
    // <peering>
    let peering::Peering {
        as_expr,
        router_expr1,
        router_expr2,
    } = mp_peering;
    let as_expr = parse_as_expr(as_expr);
    let router_expr1 = router_expr1.map(parse_router_expr);
    let router_expr2 = router_expr2.map(parse_router_expr);
    Peering::PeeringSpec {
        as_expr,
        router_expr1,
        router_expr2,
    }
}

pub fn try_parse_peering_set(mp_peering: &peering::Peering) -> Option<Peering> {
    match mp_peering {
        peering::Peering {
            as_expr: peering::AsExpr::Field(field),
            router_expr1: None,
            router_expr2: None,
        } => regex_captures!(r"^prng-(\w+)$", field)
            .map(|(_, name)| Peering::PeeringSet(name.into())),
        _ => None,
    }
}

pub fn parse_as_expr(as_expr: peering::AsExpr) -> AsExpr {
    match as_expr {
        peering::AsExpr::Field(field) => parse_as_expr_field(field),
        peering::AsExpr::AsComp(comp) => parse_complex_as_expr(comp),
    }
}

pub fn parse_complex_as_expr(comp: peering::ComplexAsExpr) -> AsExpr {
    todo!()
}

pub fn parse_as_expr_field(field: String) -> AsExpr {
    todo!()
}

// TODO: Fill in.
/// <https://www.rfc-editor.org/rfc/rfc2622#section-5.6>
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Peering {
    PeeringSpec {
        as_expr: AsExpr,
        router_expr1: Option<RouterExpr>,
        router_expr2: Option<RouterExpr>,
    },
    PeeringSet(String),
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PeeringAction {
    pub mp_peering: Peering,
    pub actions: Option<Actions>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum AsExpr {
    AsNum(usize),
    AsSet(String),
    AsComp(ComplexAsExpr),
    Illegal(String),
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
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
}
