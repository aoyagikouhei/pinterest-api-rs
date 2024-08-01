use reqwest::{RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::error::{Error, ErrorResponse};

pub mod get_boards;
pub mod get_user_account;

#[derive(Debug)]
pub struct ApiResponse<T> {
    pub body: T,
    pub status_code: StatusCode,
    pub header: HashMap<String, String>,
}

pub async fn execute_api<T>(builder: RequestBuilder) -> Result<ApiResponse<T>, Error>
where
    T: DeserializeOwned,
{
    let response = builder.send().await?;
    let status_code = response.status();
    let header = response
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
        .collect();
    let text = match response.text().await {
        Ok(text) => text,
        Err(err) => return Err(Error::Other(format!("{:?}", err), status_code)),
    };
    let json = match serde_json::from_str::<serde_json::Value>(&text) {
        Ok(json) => json,
        Err(_err) => return Err(Error::Other(text, status_code)),
    };
    if let Some(code) = json["code"].as_i64() {
        return Err(Error::Api(
            ErrorResponse {
                code,
                message: json["message"].as_str().unwrap_or_default().to_owned(),
            },
            status_code,
        ));
    }
    match serde_json::from_value::<T>(json) {
        Ok(json) => Ok(ApiResponse {
            body: json,
            status_code,
            header,
        }),
        Err(err) => Err(Error::Other(format!("{:?},{}", err, text), status_code)),
    }
}
