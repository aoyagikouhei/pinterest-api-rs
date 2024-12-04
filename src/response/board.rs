use serde::{Deserialize, Serialize};
use std::collections::HashMap as Map;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Privacy {
    Protected,
    Public,
    Secret,
}

// Privacyの表示を設定
impl std::fmt::Display for Privacy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Protected => write!(f, "PROTECTED"),
            Self::Public => write!(f, "PUBLIC"),
            Self::Secret => write!(f, "SECRET"),
        }
    }
}

// Privacyのデフォルト値を設定
impl Default for Privacy {
    fn default() -> Self {
        Self::Public
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Board {
    pub id: String,
    pub created_at: String,
    pub board_pins_modified_at: String,
    pub name: String,
    pub description: Option<String>,
    pub collaborator_count: i64,
    pub pin_count: i64,
    pub follower_count: i64,
    pub media: BoardMedia,
    pub owner: Owner,
    pub privacy: Privacy,

    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BoardMedia {
    pub image_cover_url: Option<String>,
    pub pin_thumbnail_urls: Vec<String>,

    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Owner {
    pub username: String,

    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, serde_json::Value>,
}
