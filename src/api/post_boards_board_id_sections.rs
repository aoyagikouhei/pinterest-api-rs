use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use crate::{
    api::{execute_api, ApiResponse},
    error::Error,
    options::{apply_options, make_url, ApiOptions},
    response::board_section::BoardSection,
};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    pub name: String,
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    options: Option<ApiOptions>,
    board_id: String,
    ad_account_id: Option<String>,
    body: Body,
}

impl Api {
    pub fn new(options: Option<ApiOptions>, board_id: &str, body: Body) -> Self {
        Self {
            options,
            board_id: board_id.to_string(),
            body,
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
            .post(make_url(
                &format!("/boards/{}/sections", self.board_id),
                &self.options,
            ))
            .query(&query_parameters)
            .json(&self.body)
            .bearer_auth(bearer_code);
        apply_options(client, &self.options)
    }

    pub async fn execute(self, bearer_code: &str) -> Result<ApiResponse<BoardSection>, Error> {
        execute_api(self.build(bearer_code)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // BEARER_CODE=xxx cargo test test_post_boards_board_id_sections -- --nocapture --test-threads=1

    #[tokio::test]
    async fn test_post_boards_board_id_sections() {
        let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
        let board_id = std::env::var("BOARD_ID").unwrap_or_default();
        let body = Body {
            name: "test".to_owned(),
        };
        let response = Api::new(None, &board_id, body)
            .execute(bearer_code.as_str())
            .await
            .unwrap();
        println!("{:?}", response);
    }
}