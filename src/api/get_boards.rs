use crate::error::Error;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use crate::{
    options::{apply_options, make_url, ApiOptions},
    response::board::Board,
};

use super::{execute_api, ApiResponse};

const URL_PATH: &str = "/boards";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Privacy {
    All,
    Protected,
    Public,
    PublicAndSecret,
    Secret,
}

impl std::fmt::Display for Privacy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "ALL"),
            Self::Protected => write!(f, "PROTECTED"),
            Self::Public => write!(f, "PUBLIC"),
            Self::PublicAndSecret => write!(f, "PUBLIC_AND_SECRET"),
            Self::Secret => write!(f, "SECRET"),
        }
    }
}

impl Default for Privacy {
    fn default() -> Self {
        Self::All
    }
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    options: Option<ApiOptions>,
    ad_account_id: Option<String>,
    bookmark: Option<String>,
    page_size: Option<u64>,
    privacy: Option<Privacy>,
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

    pub fn bookmark(mut self, bookmark: &str) -> Self {
        self.bookmark = Some(bookmark.to_string());
        self
    }

    pub fn page_size(mut self, page_size: u64) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn privacy(mut self, privacy: Privacy) -> Self {
        self.privacy = Some(privacy);
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
        if let Some(privacy) = self.privacy {
            query_parameters.push(("privacy", privacy.to_string()));
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
    items: Vec<Board>,
    bookmark: Option<String>,
}
