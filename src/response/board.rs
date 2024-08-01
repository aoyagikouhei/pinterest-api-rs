use serde::{Deserialize, Serialize};

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
    id: String,
    created_at: String,
    board_pins_modified_at: String,
    name: String,
    description: Option<String>,
    collaborator_count: i64,
    pin_count: i64,
    follower_count: i64,
    media: Media,
    owner: Owner,
    privacy: Privacy,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Media {
    image_cover_url: Option<String>,
    pin_thumbnail_urls: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Owner {
    username: String,
}
