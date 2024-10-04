use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use crate::{
    api::{execute_api, ApiResponse},
    error::Error,
    options::{apply_options, make_url, ApiOptions},
    response::media_response::{MediaPostResponse, MediaType},
};

const URL_PATH: &str = "/media";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    pub media_type: MediaType,
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    options: Option<ApiOptions>,
    body: Body,
}

impl Api {
    pub fn new(options: Option<ApiOptions>, body: Body) -> Self {
        Self { options, body }
    }

    pub fn build(self, bearer_code: &str) -> RequestBuilder {
        let client = reqwest::Client::new()
            .post(make_url(URL_PATH, &self.options))
            .json(&self.body)
            .bearer_auth(bearer_code);
        apply_options(client, &self.options)
    }

    pub async fn execute(self, bearer_code: &str) -> Result<ApiResponse<MediaPostResponse>, Error> {
        execute_api(self.build(bearer_code)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // BEARER_CODE=xxx cargo test test_post_media -- --nocapture --test-threads=1

    #[tokio::test]
    async fn test_post_media() {
        let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
        let body = Body::default();
        let response = Api::new(None, body)
            .execute(bearer_code.as_str())
            .await
            .unwrap();
        println!("{:?}", response);
    }
}
