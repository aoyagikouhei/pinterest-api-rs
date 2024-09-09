use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "source_type")]
#[serde(rename_all = "snake_case")]
pub enum MediaSource {
    MultipleImageBase64 {
        items: Vec<MultipleImageBase64Item>,
        index: i64,
    },
    ImageBase64 {
        content_type: String,
        data: String,
        is_standard: bool,
    },
    MultipleImageUrls {
        items: Vec<MultipleImageUrlItem>,
        index: i64,
    },
    ImageUrl {
        url: String,
        is_standard: bool,
    },
    VideoId {
        cover_image_url: String,
        cover_image_content_type: String,
        cover_image_data: String,
        media_id: String,
        is_standard: bool,
    },
    PinUrl {
        is_affiliate_link: bool,
    }
}

impl Default for MediaSource {
    fn default() -> Self {
        MediaSource::ImageUrl {
            url: "".to_string(),
            is_standard: true,
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MultipleImageBase64Item {
    pub title: String,
    pub description: String,
    pub link: String,
    pub content_type: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MultipleImageUrlItem {
    pub title: String,
    pub description: String,
    pub link: String,
    pub url: String,
}