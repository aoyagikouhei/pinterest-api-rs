use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PinFilter {
    ExcludeNative,
    ExcludeRepins,
    HasBeenPromoted,
}

impl fmt::Display for PinFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            PinFilter::ExcludeNative => "exclude_native",
            PinFilter::ExcludeRepins => "exclude_repins",
            PinFilter::HasBeenPromoted => "has_been_promoted",
        };
        write!(f, "{}", display_str)
    }
}

impl Default for PinFilter {
    fn default() -> Self {
        Self::ExcludeNative
    }
}
