use reqwest::RequestBuilder;

use crate::{
    api::{execute_api, ApiResponse},
    error::Error,
    options::{apply_options, make_url, ApiOptions},
    parameter::{app_type::AppType, metric_type::MetricTypes, split_field::SplitField}, response::analytics::Analytics,
};

#[derive(Debug, Clone, Default)]
pub struct Api {
    options: Option<ApiOptions>,
    pin_id: String,
    start_date: String,
    end_date: String,
    app_types: Option<AppType>,
    metric_types: MetricTypes,
    split_field: Option<SplitField>,
    ad_account_id: Option<String>,
}

impl Api {
    pub fn new(
        options: Option<ApiOptions>,
        pin_id: &str,
        start_date: &str,
        end_date: &str,
        metric_types: MetricTypes,
    ) -> Self {
        Self {
            options,
            pin_id: pin_id.to_string(),
            start_date: start_date.to_string(),
            end_date: end_date.to_string(),
            metric_types,
            ..Default::default()
        }
    }

    pub fn ad_account_id(mut self, ad_account_id: &str) -> Self {
        self.ad_account_id = Some(ad_account_id.to_string());
        self
    }

    pub fn app_types(mut self, app_types: AppType) -> Self {
        self.app_types = Some(app_types);
        self
    }

    pub fn split_field(mut self, split_field: SplitField) -> Self {
        self.split_field = Some(split_field);
        self
    }

    pub fn build(self, bearer_code: &str) -> RequestBuilder {
        let mut query_parameters = vec![];
        query_parameters.push(("start_date", self.start_date));
        query_parameters.push(("end_date", self.end_date));
        query_parameters.push(("metric_types", self.metric_types.to_string()));
        if let Some(app_types) = self.app_types {
            query_parameters.push(("app_types", app_types.to_string()));
        }
        if let Some(split_field) = self.split_field {
            query_parameters.push(("split_field", split_field.to_string()));
        }
        if let Some(ad_account_id) = self.ad_account_id {
            query_parameters.push(("ad_account_id", ad_account_id));
        }
        let client = reqwest::Client::new()
            .get(make_url(
                &format!("/pins/{}analytics", self.pin_id),
                &self.options,
            ))
            .query(&query_parameters)
            .bearer_auth(bearer_code);
        apply_options(client, &self.options)
    }

    pub async fn execute(self, bearer_code: &str) -> Result<ApiResponse<Analytics>, Error> {
        execute_api(self.build(bearer_code)).await
    }
}

#[cfg(test)]
mod tests {
    use crate::parameter::metric_type;

    use super::*;

    // BEARER_CODE=xxx PIN_ID=XXX cargo test test_get_pins_pin_id_analytics -- --nocapture --test-threads=1

    #[tokio::test]
    async fn test_get_pins_pin_id_analytics() {
        let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
        let pin_id = std::env::var("PIN_ID").unwrap_or_default();
        let metric_types =
            metric_type::MetricTypes::Standard(vec![metric_type::StandardMetricType::Impression]);
        let response = Api::new(None, &pin_id, "2000-01-01", "2001-01-01", metric_types)
            .execute(bearer_code.as_str())
            .await
            .unwrap();
        println!("{:?}", response);
    }
}
