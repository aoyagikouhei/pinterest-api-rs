use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    Image,
    Video,
    MultipleImages,
    MultipleVideos,
    MultipleMixed,
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Image => write!(f, "image"),
            Self::Video => write!(f, "video"),
            Self::MultipleImages => write!(f, "multiple_images"),
            Self::MultipleVideos => write!(f, "multiple_videos"),
            Self::MultipleMixed => write!(f, "multiple_mixed"),
        }
    }
}

impl Default for MediaType {
    fn default() -> Self {
        Self::Image
    }
}