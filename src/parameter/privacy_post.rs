use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrivacyPost {
    Protected,
    Public,
    Secret,
}

impl std::fmt::Display for PrivacyPost {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Protected => write!(f, "PROTECTED"),
            Self::Public => write!(f, "PUBLIC"),
            Self::Secret => write!(f, "SECRET"),
        }
    }
}

impl Default for PrivacyPost {
    fn default() -> Self {
        Self::Public
    }
}
