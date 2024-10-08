use reqwest::RequestBuilder;

use crate::{
    api::{execute_api, ApiResponse},
    error::Error,
    options::{apply_options, make_url, ApiOptions},
    response::{board_section::BoardSection, list_response::ListResponse},
};

#[derive(Debug, Clone, Default)]
pub struct Api {
    options: Option<ApiOptions>,
    board_id: String,
    ad_account_id: Option<String>,
    bookmark: Option<String>,
    page_size: Option<u64>,
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
        if let Some(ad_account_id) = self.ad_account_id {
            query_parameters.push(("ad_account_id", ad_account_id));
        }
        if let Some(bookmark) = self.bookmark {
            query_parameters.push(("bookmark", bookmark));
        }
        if let Some(page_size) = self.page_size {
            query_parameters.push(("page_size", page_size.to_string()));
        }
        let client = reqwest::Client::new()
            .get(make_url(
                &format!("/boards/{}/sections", self.board_id),
                &self.options,
            ))
            .query(&query_parameters)
            .bearer_auth(bearer_code);
        apply_options(client, &self.options)
    }

    pub async fn execute(
        self,
        bearer_code: &str,
    ) -> Result<ApiResponse<ListResponse<BoardSection>>, Error> {
        execute_api(self.build(bearer_code)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // BEARER_CODE=xxx BOARD_ID=xxx cargo test test_get_boards_board_id_sections -- --nocapture --test-threads=1

    #[tokio::test]
    async fn test_get_boards_board_id_sections() {
        let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
        let board_id = std::env::var("BOARD_ID").unwrap_or_default();
        let response = Api::new(None, &board_id)
            .execute(bearer_code.as_str())
            .await
            .unwrap();
        println!("{:?}", response);
    }
}
