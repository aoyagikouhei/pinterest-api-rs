use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAccount {
    account_type: String,
    id: String,
    profile_image: String,
    website_url: String,
    username: String,
    about: String,
    business_name: Option<String>,
    board_count: Option<i64>,
    pin_count: Option<i64>,
    follower_count: Option<i64>,
    following_count: Option<i64>,
    monthly_views: Option<i64>,
}
