use reqwest::RequestBuilder;

use crate::{
    error::Error,
    options::{apply_options, make_url, ApiOptions},
};

use super::{execute_empty_api, ApiEmptyResponse};

const URL_PATH: &str = "/boards";

#[derive(Debug, Clone, Default)]
pub struct Api {
    options: Option<ApiOptions>,
    board_id: String,
    ad_account_id: Option<String>,
}

impl Api {
    pub fn new(options: Option<ApiOptions>, board_id: &str) -> Self {
        Self {
            options,
            board_id: board_id.to_string(),
            ..Default::default()
        }
    }

    pub fn ad_account_id(mut self, ad_account_id: &str) -> Self {
        self.ad_account_id = Some(ad_account_id.to_string());
        self
    }

    pub fn build(self, bearer_code: &str) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(ad_account_id) = self.ad_account_id {
            query_parameters.push(("ad_account_id", ad_account_id));
        }
        let client = reqwest::Client::new()
            .delete(make_url(
                &format!("{}/{}", URL_PATH, self.board_id),
                &self.options,
            ))
            .query(&query_parameters)
            .bearer_auth(bearer_code);
        apply_options(client, &self.options)
    }

    pub async fn execute(self, bearer_code: &str) -> Result<ApiEmptyResponse, Error> {
        execute_empty_api(self.build(bearer_code)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // BEARER_CODE=xxx BOARD_ID=xxx cargo test test_delete_boards_board_id -- --nocapture --test-threads=1

    #[tokio::test]
    async fn test_delete_boards_board_id() {
        let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
        let board_id = std::env::var("BOARD_ID").unwrap_or_default();
        let response = Api::new(None, &board_id)
            .execute(bearer_code.as_str())
            .await
            .unwrap();
        println!("{:?}", response);
    }
}
