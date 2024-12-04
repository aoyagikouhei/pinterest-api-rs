use super::images::Images;
use serde::{Deserialize, Serialize};
use std::collections::HashMap as Map;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "item_type")]
#[serde(rename_all = "snake_case")]
pub enum MixedItem {
    Image {
        title: Option<String>,
        description: Option<String>,
        link: Option<String>,
        images: Box<Images>,

        #[serde(flatten, skip_serializing_if = "Map::is_empty")]
        extra: Map<String, serde_json::Value>,
    },
    Video {
        cover_image_url: String,
        video_url: Option<String>,
        duration: i64,
        height: i64,
        width: i64,

        #[serde(flatten, skip_serializing_if = "Map::is_empty")]
        extra: Map<String, serde_json::Value>,
    },
}
