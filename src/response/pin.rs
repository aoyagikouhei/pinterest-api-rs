use super::media::Media;
use crate::response::creative_type::CreativeType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap as Map;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pin {
    pub id: String,
    pub created_at: String,
    pub link: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub dominant_color: Option<String>,
    pub alt_text: Option<String>,
    pub creative_type: Option<CreativeType>,
    pub board_id: String,
    pub board_section_id: Option<String>,
    pub board_owner: BoardOwner,
    pub is_owner: bool,
    pub media: Media,
    pub parent_pin_id: Option<String>,
    pub is_standard: bool,
    pub has_been_promoted: bool,
    pub note: Option<String>,
    pub pin_metrics: Option<serde_json::Value>,

    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoardOwner {
    pub username: String,

    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, serde_json::Value>,
}
