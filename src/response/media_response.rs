use serde::{Deserialize, Serialize};
use std::collections::HashMap as Map;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Video,
}

impl std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Video => write!(f, "video"),
        }
    }
}

impl Default for MediaType {
    fn default() -> Self {
        Self::Video
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Registered,
    Processing,
    Succeeded,
    Failed,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Registered => write!(f, "registered"),
            Self::Processing => write!(f, "processing"),
            Self::Succeeded => write!(f, "succeeded"),
            Self::Failed => write!(f, "failed"),
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::Registered
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UploadParameters {
    #[serde(rename = "x-amz-date")]
    pub x_amz_date: String,
    #[serde(rename = "x-amz-signature")]
    pub x_amz_signature: String,
    #[serde(rename = "x-amz-security-token")]
    pub x_amz_security_token: String,
    #[serde(rename = "x-amz-algorithm")]
    pub x_amz_algorithm: String,
    pub key: String,
    pub policy: String,
    #[serde(rename = "x-amz-credential")]
    pub x_amz_credential: String,
    #[serde(rename = "Content-Type")]
    pub content_type: String,

    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MediaGetResponse {
    pub media_id: String,
    pub media_type: MediaType,
    pub status: Status,

    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MediaPostResponse {
    pub media_id: String,
    pub media_type: MediaType,
    pub upload_url: Status,
    pub upload_parameters: UploadParameters,

    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, serde_json::Value>,
}
