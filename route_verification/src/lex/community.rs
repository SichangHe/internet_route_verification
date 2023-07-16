use super::*;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Call {
    pub method: Option<String>,
    pub args: Vec<String>,
}
