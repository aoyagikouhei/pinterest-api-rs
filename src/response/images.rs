use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Images {
    #[serde(rename = "150x150")]
    pub size_150x150: Image,
    #[serde(rename = "400x300")]
    pub size_400x300: Image,
    #[serde(rename = "600x")]
    pub size_600x: Image,
    #[serde(rename = "1200x")]
    pub size_1200x: Image,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Image {
    pub width: i64,
    pub height: i64,
    pub url: String,
}
