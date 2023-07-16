use super::*;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AutNum {
    pub name: String,
    pub body: String,
    pub imports: Versions,
    pub exports: Versions,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AsOrRouteSet {
    pub name: String,
    pub body: String,
    pub members: Vec<String>,
}

impl AsOrRouteSet {
    pub fn new(name: String, body: String, members: Vec<String>) -> Self {
        Self {
            name,
            body,
            members,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PeeringSet {
    pub name: String,
    pub body: String,
    pub peerings: Vec<Peering>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct FilterSet {
    pub name: String,
    pub body: String,
    pub filters: Vec<Filter>,
}
