use reqwest::RequestBuilder;

use crate::{
    api::{execute_api, ApiResponse},
    error::Error,
    options::{apply_options, make_url, ApiOptions},
    response::{list_response::ListResponse, media_response::MediaGetResponse},
};

const URL_PATH: &str = "/media";

#[derive(Debug, Clone, Default)]
pub struct Api {
    options: Option<ApiOptions>,
    bookmark: Option<String>,
    page_size: Option<u64>,
}

impl Api {
    pub fn new(options: Option<ApiOptions>) -> Self {
        Self {
            options,
            ..Default::default()
        }
    }

    pub fn bookmark(mut self, bookmark: &str) -> Self {
        self.bookmark = Some(bookmark.to_string());
        self
    }

    pub fn page_size(mut self, page_size: u64) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn build(self, bearer_code: &str) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(bookmark) = self.bookmark {
            query_parameters.push(("bookmark", bookmark));
        }
        if let Some(page_size) = self.page_size {
            query_parameters.push(("page_size", page_size.to_string()));
        }
        let client = reqwest::Client::new()
            .get(make_url(URL_PATH, &self.options))
            .query(&query_parameters)
            .bearer_auth(bearer_code);
        apply_options(client, &self.options)
    }

    pub async fn execute(
        self,
        bearer_code: &str,
    ) -> Result<ApiResponse<ListResponse<MediaGetResponse>>, Error> {
        execute_api(self.build(bearer_code)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // BEARER_CODE=xxx cargo test test_get_media -- --nocapture --test-threads=1

    #[tokio::test]
    async fn test_get_media() {
        let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
        let response = Api::new(None).execute(bearer_code.as_str()).await.unwrap();
        println!("{:?}", response);
    }
}
