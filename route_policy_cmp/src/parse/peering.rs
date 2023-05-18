use lazy_regex::{regex_captures, regex_is_match};
use log::error;
use serde::{Deserialize, Serialize};

use crate::lex::{mp_import, peering};

use super::{
    action::{parse_actions, Actions},
    lex::parse_aut_num_name,
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
    let actions = parse_actions(actions);
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
    let remote_as = parse_as_expr(as_expr);
    let remote_router = router_expr1.map(parse_router_expr);
    let local_router = router_expr2.map(parse_router_expr);
    Peering::PeeringSpec {
        remote_as,
        remote_router,
        local_router,
    }
}

pub fn try_parse_peering_set(mp_peering: &peering::Peering) -> Option<Peering> {
    match mp_peering {
        peering::Peering {
            as_expr: peering::AsExpr::Field(field),
            router_expr1: None,
            router_expr2: None,
        } => regex_is_match!(r"^(AS\d+:)?prng-\S+$"i, field)
            .then(|| Peering::PeeringSet(field.into())),
        _ => None,
    }
}

pub fn parse_as_expr(as_expr: peering::AsExpr) -> AsExpr {
    match as_expr {
        peering::AsExpr::Field(field) => parse_as_expr_field(&field),
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

/// A simple AS field is either a AS number or a AS set.
/// Otherwise, return `AsExpr::Illegal`.
pub fn parse_as_expr_field(field: &str) -> AsExpr {
    if let Some(name) = try_parse_as_set(field) {
        // AS set.
        return AsExpr::AsSet(name.into());
    }
    match parse_aut_num_name(field) {
        Ok(num) => AsExpr::AsNum(num), // AS number.
        Err(err) => {
            error!("{err}");
            AsExpr::Illegal(err.to_string())
        }
    }
}

pub fn try_parse_as_set(field: &str) -> Option<&str> {
    regex_captures!(r"^AS-(\S+)$"i, field).map(|(_, name)| name)
}

/// <https://www.rfc-editor.org/rfc/rfc2622#section-5.6>
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Peering {
    PeeringSpec {
        remote_as: AsExpr,
        remote_router: Option<RouterExpr>,
        local_router: Option<RouterExpr>,
    },
    PeeringSet(String),
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PeeringAction {
    pub mp_peering: Peering,
    pub actions: Actions,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AsExpr {
    AsNum(usize),
    AsSet(String),
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
    Illegal(String),
}
