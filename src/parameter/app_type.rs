use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppType {
    All,
    Mobile,
    Tablet,
    Web,
}

impl fmt::Display for AppType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::All => write!(f, "ALL"),
            Self::Mobile => write!(f, "MOBILE"),
            Self::Tablet => write!(f, "TABLET"),
            Self::Web => write!(f, "WEB"),
        }
    }
}

impl Default for AppType {
    fn default() -> Self {
        Self::All
    }
}
