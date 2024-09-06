use serde::{Deserialize, Serialize};

use super::{images::Images, mixed_item::MixedItem,};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "media_type")]
#[serde(rename_all = "snake_case")]
pub enum Media {
    Image{
        images: Images,
    },
    Video {
        images: Images,
        cover_image_url: String,
        video_url: Option<String>,
        duration: i64,
        hegith: i64,
        width: i64,
    },
    MultipleImages {
        items: Vec<MixedItem>,
        images: Images,
    },
    MultipleVideos {
        items: Vec<MixedItem>,
        images: Images,
    },
    MultipleMixed {
        items: Vec<MixedItem>,
        images: Images,
    },

}

#[cfg(test)]
mod tests {
    use super::*;

    // BEARER_CODE=xxx cargo test test_response_media_json -- --nocapture --test-threads=1

    #[tokio::test]
    async fn test_response_media_json() {
        let image = Media::Image{images: Images{..Default::default()}};
        println!("{}", serde_json::to_string(&image).unwrap());

        let multiple_images = Media::MultipleImages { items: vec![], images: Images{..Default::default()} };
        println!("{}", serde_json::to_string(&multiple_images).unwrap());
    }
}

