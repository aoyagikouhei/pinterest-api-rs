use serde::{Deserialize, Serialize};
use std::collections::HashMap as Map;

use super::{images::Images, mixed_item::MixedItem};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "media_type")]
#[serde(rename_all = "snake_case")]
pub enum Media {
    Image {
        images: Box<Images>,

        #[serde(flatten, skip_serializing_if = "Map::is_empty")]
        extra: Map<String, serde_json::Value>,
    },
    Video {
        images: Box<Images>,
        cover_image_url: String,
        video_url: Option<String>,
        duration: Option<i64>,
        hegith: Option<i64>,
        width: Option<i64>,

        #[serde(flatten, skip_serializing_if = "Map::is_empty")]
        extra: Map<String, serde_json::Value>,
    },
    MultipleImages {
        items: Vec<MixedItem>,
        images: Box<Images>,

        #[serde(flatten, skip_serializing_if = "Map::is_empty")]
        extra: Map<String, serde_json::Value>,
    },
    MultipleVideos {
        items: Vec<MixedItem>,
        images: Box<Images>,

        #[serde(flatten, skip_serializing_if = "Map::is_empty")]
        extra: Map<String, serde_json::Value>,
    },
    MultipleMixed {
        items: Vec<MixedItem>,
        images: Box<Images>,

        #[serde(flatten)]
        extra: Map<String, serde_json::Value>,
    },
}

impl Default for Media {
    fn default() -> Self {
        Self::Image {
            images: Box::new(Images {
                ..Default::default()
            }),
            extra: Map::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    // BEARER_CODE=xxx cargo test test_response_media_json -- --nocapture --test-threads=1

    #[tokio::test]
    async fn test_response_media_json() {
        let image = Media::Image {
            images: Box::new(Images {
                ..Default::default()
            }),
            extra: Map::new(),
        };
        println!("{}", serde_json::to_string(&image).unwrap());

        let multiple_images = Media::MultipleImages {
            items: vec![],
            images: Box::new(Images {
                ..Default::default()
            }),
            extra: Map::new(),
        };
        println!("{}", serde_json::to_string(&multiple_images).unwrap());

        let json = json!({
            "media_type": "image",
            "images": {
                "150x150": {
                    "width": 150,
                    "height": 150,
                    "url": "https://example.com/150x150.jpg"
                },
                "400x300": {
                    "width": 150,
                    "height": 150,
                    "url": "https://example.com/150x150.jpg"
                },
                "600x": {
                    "width": 150,
                    "height": 150,
                    "url": "https://example.com/150x150.jpg"
                },
                "1200x": {
                    "width": 150,
                    "height": 150,
                    "url": "https://example.com/150x150.jpg"
                },
            },
            "aaa": "bbb"
        });

        let image: Media = serde_json::from_value(json).unwrap();
        println!("{:?}", image);
    }
}
