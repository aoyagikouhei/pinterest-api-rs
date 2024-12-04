use serde::{Deserialize, Serialize};
use std::collections::HashMap as Map;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAccount {
    pub account_type: String,
    pub id: String,
    pub profile_image: String,
    pub website_url: String,
    pub username: String,
    pub about: String,
    pub business_name: Option<String>,
    pub board_count: Option<i64>,
    pub pin_count: Option<i64>,
    pub follower_count: Option<i64>,
    pub following_count: Option<i64>,
    pub monthly_views: Option<i64>,

    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, serde_json::Value>,
}
