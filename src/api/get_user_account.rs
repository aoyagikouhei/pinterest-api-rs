use crate::{
    api::{execute_api, ApiResponse},
    error::Error,
    options::{apply_options, make_url, ApiOptions},
    response::user_account::UserAccount,
};
use reqwest::RequestBuilder;

const URL_PATH: &str = "/user_account";

#[derive(Debug, Clone, Default)]
pub struct Api {
    options: Option<ApiOptions>,
    ad_account_id: Option<String>,
}

impl Api {
    pub fn new(options: Option<ApiOptions>) -> Self {
        Self {
            options,
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
            .get(make_url(URL_PATH, &self.options))
            .query(&query_parameters)
            .bearer_auth(bearer_code);
        apply_options(client, &self.options)
    }

    pub async fn execute(self, bearer_code: &str) -> Result<ApiResponse<UserAccount>, Error> {
        execute_api(self.build(bearer_code)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // BEARER_CODE=xxx cargo test test_get_user_account -- --nocapture --test-threads=1

    #[tokio::test]
    async fn test_get_user_account() {
        let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
        let response = Api::new(None)
            .execute(bearer_code.as_str())
            .await
            .unwrap();
        println!("{:?}", response);
    }
}