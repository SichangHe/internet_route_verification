use std::convert::identity;

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

pub fn is_peering_set(field: &str) -> bool {
    regex!(formatcp!("^{}$", PEERING_SET)).is_match(field)
}

pub fn parse_as_expr(as_expr: peering::AsExpr) -> AsExpr {
    match as_expr {
        peering::AsExpr::Field(single) => parse_single_as_expr(single),
        peering::AsExpr::AsComp(comp) => parse_complex_as_expr(comp),
    }
}

pub fn parse_single_as_expr(single: String) -> AsExpr {
    if is_peering_set(&single) {
        AsExpr::PeeringSet(single)
    } else {
        AsExpr::Single(
            parse_as_name(single).map_or_else(|e| AsName::Invalid(e.to_string()), identity),
        )
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

/// >  The syntax of a peering specification is:
/// >
/// >  <as-expression> [<router-expression-1>] [at <router-expression-2>]
/// > | <peering-set-name>
/// >
/// >  where <as-expression> is an expression over AS numbers and AS sets
/// >  using operators AND, OR, and EXCEPT, and <router-expression-1> and
/// >  <router-expression-2> are expressions over router IP addresses,
/// >  inet-rtr names, and rtr-set names using operators AND, OR, and
/// >  EXCEPT.  The binary "EXCEPT" operator is the set subtraction
/// >  operator and has the same precedence as the operator AND (it is
/// >  semantically equivalent to "AND NOT" combination).  That is "(AS1
/// >  OR AS2) EXCEPT AS2" equals "AS1".
/// >
/// >  This form identifies all the peerings between any local router in
/// >  <router-expression-2> to any of their peer routers in <router-
/// >  expression-1> in the ASes in <as-expression>.  If <router-
/// >  expression-2> is not specified, it defaults to all routers of the
/// >  local AS that peer with ASes in <as-expression>.  If <router-
/// >  expression-1> is not specified, it defaults to all routers of the
/// >  peer ASes in <as-expression> that peer with the local AS.
///
/// <https://www.rfc-editor.org/rfc/rfc2622#section-5.6>
#[derive(Clone, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Peering {
    pub remote_as: AsExpr,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub remote_router: Option<RouterExpr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub local_router: Option<RouterExpr>,
}

impl std::fmt::Debug for Peering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut r = f.debug_struct("Peering");
        r.field("remote_as", &self.remote_as);
        for (name, field) in [
            ("remote_router", &self.remote_router),
            ("local_router", &self.local_router),
        ] {
            if let Some(field) = field {
                r.field(name, field);
            }
        }
        r.finish()
    }
}

#[derive(Clone, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PeeringAction {
    pub mp_peering: Peering,
    #[serde(default)]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub actions: Actions,
}

impl std::fmt::Debug for PeeringAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut r = f.debug_struct("PeeringAction");
        r.field("mp_peering", &self.mp_peering);
        if !self.actions.is_empty() {
            r.field("actions", &self.actions);
        }
        r.finish()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AsExpr {
    Single(AsName),
    PeeringSet(String),
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
