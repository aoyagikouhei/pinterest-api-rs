use serde::{Deserialize, Serialize};

use super::{images::Images, media_type::MediaType};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Media {
    Image{
        images: Images,
        media_type: MediaType,
    },
    Video {
        images: Images,
        media_type: MediaType,
        cover_image_url: String,
        video_url: Option<String>,
        duration: i64,
        hegith: i64,
        width: i64,
    },
    MultipleImages {
        items: Vec<ImageItem>,
        images: Vec<Images>,
        media_type: MediaType,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageItem {
    item_type: String,
    title: Option<String>,
    description: Option<String>,
    link: Option<String>,
    images: Images,
}



