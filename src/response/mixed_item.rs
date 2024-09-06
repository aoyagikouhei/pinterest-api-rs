use serde::{Deserialize, Serialize};

use super::images::Images;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "item_type")]
#[serde(rename_all = "snake_case")]
pub enum MixedItem {
    Image {
        title: Option<String>,
        description: Option<String>,
        link: Option<String>,
        images: Images,
    },
    Video {
        cover_image_url: String,
        video_url: Option<String>,
        duration: i64,
        height: i64,
        width: i64,
    }
}
