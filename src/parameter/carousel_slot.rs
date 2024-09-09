use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CarouselSlot {
    pub title: String,
    pub description: String,
    pub link: String,
}
