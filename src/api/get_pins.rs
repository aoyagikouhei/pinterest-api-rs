use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use crate::{
    api::{execute_api, ApiResponse},
    error::Error,
    options::{apply_options, make_url, ApiOptions},
    parameter::{creative_type::CreativeType, pin_filter::PinFilter, pin_type::PinType,},
    response::board::Board,
};

const URL_PATH: &str = "/pins";

#[derive(Debug, Clone, Default)]
pub struct Api {
    options: Option<ApiOptions>,
    bookmark: Option<String>,
    page_size: Option<u64>,
    pin_filter: Option<PinFilter>,
    include_protected_pins: Option<bool>,
    pin_type: Option<PinType>,
    cretive_types: Option<Vec<CreativeType>>,
    ad_account_id: Option<String>,
    pin_metrics: Option<bool>,
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

    pub fn pin_filter(mut self, pin_filter: PinFilter) -> Self {
        self.pin_filter = Some(pin_filter);
        self
    }

    pub fn include_protected_pins(mut self, include_protected_pins: bool) -> Self {
        self.include_protected_pins = Some(include_protected_pins);
        self
    }

    pub fn pin_type(mut self, pin_type: PinType) -> Self {
        self.pin_type = Some(pin_type);
        self
    }

    pub fn creative_types(mut self, creative_types: Vec<CreativeType>) -> Self {
        self.cretive_types = Some(creative_types);
        self
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
        if let Some(bookmark) = self.bookmark {
            query_parameters.push(("bookmark", bookmark));
        }
        if let Some(page_size) = self.page_size {
            query_parameters.push(("page_size", page_size.to_string()));
        }
        if let Some(pin_filter) = self.pin_filter {
            query_parameters.push(("pin_filter", pin_filter.to_string()));
        }
        if let Some(include_protected_pins) = self.include_protected_pins {
            query_parameters.push(("include_protected_pins", include_protected_pins.to_string()));
        }
        if let Some(pin_type) = self.pin_type {
            query_parameters.push(("pin_type", pin_type.to_string()));
        }
        if let Some(creative_types) = self.cretive_types {
            query_parameters.push(("creative_types", creative_types.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")));
        }
        if let Some(pin_metrics) = self.pin_metrics {
            query_parameters.push(("pin_metrics", pin_metrics.to_string()));
        }
        let client = reqwest::Client::new()
            .get(make_url(URL_PATH, &self.options))
            .query(&query_parameters)
            .bearer_auth(bearer_code);
        apply_options(client, &self.options)
    }

    pub async fn execute(self, bearer_code: &str) -> Result<ApiResponse<Response>, Error> {
        execute_api(self.build(bearer_code)).await
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub items: Vec<Board>,
    pub bookmark: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // BEARER_CODE=xxx cargo test test_get_boards -- --nocapture --test-threads=1

    #[tokio::test]
    async fn test_get_boards() {
        let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
        let response = Api::new(None)
            .execute(bearer_code.as_str())
            .await
            .unwrap();
        println!("{:?}", response);
    }
}