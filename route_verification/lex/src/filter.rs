use super::*;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Filter {
    Any,
    And {
        left: Box<Filter>,
        right: Box<Filter>,
    },
    Or {
        left: Box<Filter>,
        right: Box<Filter>,
    },
    Not(Box<Filter>),
    Group(Box<Filter>),
    Community(Call),
    PathAttr(String),
    AddrPrefixSet(Vec<String>),
    Regex(String),
}

impl Default for Filter {
    fn default() -> Self {
        Self::Any
    }
}
