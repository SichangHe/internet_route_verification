use super::*;

pub type Actions = BTreeMap<String, Action>;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum Action {
    Assigned(String),
    AssignedSet(Vec<String>),
    MethodCall(Vec<Call>),
}
