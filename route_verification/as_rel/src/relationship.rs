use super::*;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Relationship {
    P2C,
    P2P,
    C2P,
}

impl FromStr for Relationship {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "-1" => P2C,
            "0" => P2P,
            "1" => C2P,
            other => bail!("invalid AS relationship `{other}`"),
        })
    }
}

impl Relationship {
    pub fn reversed(self) -> Self {
        match self {
            P2C => C2P,
            P2P => P2P,
            C2P => P2C,
        }
    }
}
