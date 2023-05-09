use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Call {
    pub method: Option<String>,
    pub args: Vec<String>,
}
