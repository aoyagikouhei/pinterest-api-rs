use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CreativeType {
    Regular,
    Video,
    Shopping,
    Carousel,
    MaxVideo,
    ShopThePin,
    Collection,
    Idea,
    Showcase,
    Quiz,
}

impl fmt::Display for CreativeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Regular => write!(f, "REGULAR"),
            Self::Video => write!(f, "VIDEO"),
            Self::Shopping => write!(f, "SHOPPING"),
            Self::Carousel => write!(f, "CAROUSEL"),
            Self::MaxVideo => write!(f, "MAX_VIDEO"),
            Self::ShopThePin => write!(f, "SHOP_THE_PIN"),
            Self::Collection => write!(f, "COLLECTION"),
            Self::Idea => write!(f, "IDEA"),
            Self::Showcase => write!(f, "SHOWCASE"),
            Self::Quiz => write!(f, "QUIZ"),
        }
    }
}

impl Default for CreativeType {
    fn default() -> Self {
        Self::Regular
    }
}