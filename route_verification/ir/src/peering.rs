use std::convert::identity;

use super::*;

pub fn is_peering_set(field: &str) -> bool {
    regex!(formatcp!("^{}$", PEERING_SET)).is_match(field)
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

/// > ```ignore
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
/// > ```
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

/// Representation of `<mp-peering> [<actions>]` in an RPSL rule.
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
