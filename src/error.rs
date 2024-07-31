use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Timeout")]
    Timeout,

    #[error("Other {0}, {1}")]
    Other(String, StatusCode),

    #[error("OAuth {0:?}, {1}")]
    Api(ErrorResponse, StatusCode),

    #[error("reqwest {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("serde json {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: i64,
    pub message: String,
}