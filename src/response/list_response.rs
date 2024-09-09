use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListResponse<T> {
    pub items: Vec<T>,
    pub bookmark: Option<String>,
}
