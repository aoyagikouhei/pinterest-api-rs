use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SplitField {
    NoSplit,
    AppType,
}

impl fmt::Display for SplitField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoSplit => write!(f, "NO_SPLIT"),
            Self::AppType => write!(f, "APP_TYPE"),
        }
    }
}

impl Default for SplitField {
    fn default() -> Self {
        Self::NoSplit
    }
}
