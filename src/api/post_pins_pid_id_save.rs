use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use crate::{
    api::{execute_api, ApiResponse},
    error::Error,
    options::{apply_options, make_url, ApiOptions},
    response::pin::Pin,
};

const URL_PATH: &str = "/pins";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    pub board_id: Option<String>,
    pub board_section_id: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    options: Option<ApiOptions>,
    pin_id: String,
    ad_account_id: Option<String>,
    body: Body,
}

impl Api {
    pub fn new(options: Option<ApiOptions>, body: Body, pin_id: &str) -> Self {
        Self {
            options,
            body,
            pin_id: pin_id.to_string(),
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
            .patch(make_url(
                &format!("{}/{}/save", URL_PATH, self.pin_id),
                &self.options,
            ))
            .query(&query_parameters)
            .json(&self.body)
            .bearer_auth(bearer_code);
        apply_options(client, &self.options)
    }

    pub async fn execute(self, bearer_code: &str) -> Result<ApiResponse<Pin>, Error> {
        execute_api(self.build(bearer_code)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // BEARER_CODE=xxx cargo test test_post_pins_pin_id_save -- --nocapture --test-threads=1

    #[tokio::test]
    async fn test_post_pins_pin_id_save() {
        let bearer_code: String = std::env::var("BEARER_CODE").unwrap_or_default();
        let pin_id = std::env::var("PIN_ID").unwrap_or_default();
        let body = Body {
            ..Default::default()
        };
        let option = ApiOptions {
            prefix_url: Some("https://api-sandbox.pinterest.com/v5".to_string()),
            ..Default::default()
        };
        let response = Api::new(Some(option), body, &pin_id)
            .execute(bearer_code.as_str())
            .await
            .unwrap();
        println!("{:?}", response);
    }
}
