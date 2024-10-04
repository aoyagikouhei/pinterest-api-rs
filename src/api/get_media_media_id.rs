use reqwest::RequestBuilder;

use crate::{
    api::{execute_api, ApiResponse},
    error::Error,
    options::{apply_options, make_url, ApiOptions},
    response::media_response::MediaGetResponse,
};

const URL_PATH: &str = "/media";

#[derive(Debug, Clone, Default)]
pub struct Api {
    options: Option<ApiOptions>,
    media_id: String,
}

impl Api {
    pub fn new(options: Option<ApiOptions>, media_id: &str) -> Self {
        Self {
            options,
            media_id: media_id.to_string(),
        }
    }

    pub fn build(self, bearer_code: &str) -> RequestBuilder {
        let client = reqwest::Client::new()
            .get(make_url(
                &format!("{}/{}", URL_PATH, self.media_id),
                &self.options,
            ))
            .bearer_auth(bearer_code);
        apply_options(client, &self.options)
    }

    pub async fn execute(self, bearer_code: &str) -> Result<ApiResponse<MediaGetResponse>, Error> {
        execute_api(self.build(bearer_code)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // BEARER_CODE=xxx MEDIA_ID=xxx cargo test test_get_media_media_id -- --nocapture --test-threads=1

    #[tokio::test]
    async fn test_get_media_media_id() {
        let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
        let media_id = std::env::var("MEDIA_ID").unwrap_or_default();
        let response = Api::new(None, &media_id)
            .execute(bearer_code.as_str())
            .await
            .unwrap();
        println!("{:?}", response);
    }
}
