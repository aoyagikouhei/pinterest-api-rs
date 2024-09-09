use reqwest::RequestBuilder;

use crate::{
    api::{execute_api, ApiResponse},
    error::Error,
    options::{apply_options, make_url, ApiOptions},
    response::pin::Pin,
};

const URL_PATH: &str = "/pins";

#[derive(Debug, Clone, Default)]
pub struct Api {
    options: Option<ApiOptions>,
    pin_id: String,
    ad_account_id: Option<String>,
    pin_metrics: Option<bool>,
}

impl Api {
    pub fn new(options: Option<ApiOptions>, pin_id: &str) -> Self {
        Self {
            options,
            pin_id: pin_id.to_string(),
            ..Default::default()
        }
    }

    pub fn ad_account_id(mut self, ad_account_id: &str) -> Self {
        self.ad_account_id = Some(ad_account_id.to_string());
        self
    }

    pub fn pin_metrics(mut self, pin_metrics: bool) -> Self {
        self.pin_metrics = Some(pin_metrics);
        self
    }

    pub fn build(self, bearer_code: &str) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(ad_account_id) = self.ad_account_id {
            query_parameters.push(("ad_account_id", ad_account_id));
        }
        if let Some(pin_metrics) = self.pin_metrics {
            query_parameters.push(("pin_metrics", pin_metrics.to_string()));
        }
        let client = reqwest::Client::new()
            .get(make_url(
                &format!("{}/{}", URL_PATH, self.pin_id),
                &self.options,
            ))
            .query(&query_parameters)
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

    // BEARER_CODE=xxx PIN_ID=904590275147080915 cargo test test_get_pins_pin_id -- --nocapture --test-threads=1

    #[tokio::test]
    async fn test_get_pins_pin_id() {
        let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
        let pin_id = std::env::var("PIN_ID").unwrap_or_default();
        let response = Api::new(None, &pin_id)
            .execute(bearer_code.as_str())
            .await
            .unwrap();
        println!("{:?}", response);
    }
}
