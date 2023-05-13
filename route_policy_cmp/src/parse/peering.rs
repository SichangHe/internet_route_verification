use serde::{Deserialize, Serialize};

use crate::lex::mp_import;

use super::action::Actions;

pub fn parse_mp_peerings(mp_peerings: Vec<mp_import::PeeringAction>) -> Vec<PeeringAction> {
    todo!()
}

// TODO: Fill in.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Peering {}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PeeringAction {
    pub mp_peering: Peering,
    pub actions: Option<Actions>,
}
