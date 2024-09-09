use reqwest::{RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::error::{Error, ErrorResponse};

pub mod delete_pins_pin_id;
pub mod get_boards;
pub mod get_boards_board_id;
pub mod get_pins;
pub mod get_pins_pin_id;
pub mod get_user_account;
pub mod patch_boards_board_id;
pub mod patch_pins_pid_id;
pub mod post_boards;
pub mod post_pins;
pub mod post_pins_pid_id_save;

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

#[derive(Debug)]
pub struct ApiEmptyResponse {
    pub status_code: StatusCode,
    pub header: HashMap<String, String>,
}

pub async fn execute_empty_api(builder: RequestBuilder) -> Result<ApiEmptyResponse, Error> {
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
    if text.is_empty() {
        return Ok(ApiEmptyResponse {
            status_code,
            header,
        });
    }
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
    Err(Error::Other(text, status_code))
}
