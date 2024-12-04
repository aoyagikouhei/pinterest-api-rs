use serde::{Deserialize, Serialize};
use std::collections::HashMap as Map;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BoardSection {
    pub id: String,
    pub name: String,

    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, serde_json::Value>,
}
