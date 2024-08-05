use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PinType {
    Private,
}

impl fmt::Display for PinType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            Self::Private => "PRIVATE",
        };
        write!(f, "{}", display_str)
    }
}

impl Default for PinType {
    fn default() -> Self {
        Self::Private
    }
}
