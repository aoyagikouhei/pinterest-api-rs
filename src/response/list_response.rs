use serde::{Deserialize, Serialize};
use std::collections::HashMap as Map;

#[derive(Serialize, Deserialize, Debug)]
pub struct ListResponse<T> {
    pub items: Vec<T>,
    pub bookmark: Option<String>,

    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, serde_json::Value>,
}
