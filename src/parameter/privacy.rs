use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Privacy {
    All,
    Protected,
    Public,
    PublicAndSecret,
    Secret,
}

impl std::fmt::Display for Privacy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "ALL"),
            Self::Protected => write!(f, "PROTECTED"),
            Self::Public => write!(f, "PUBLIC"),
            Self::PublicAndSecret => write!(f, "PUBLIC_AND_SECRET"),
            Self::Secret => write!(f, "SECRET"),
        }
    }
}

impl Default for Privacy {
    fn default() -> Self {
        Self::All
    }
}
