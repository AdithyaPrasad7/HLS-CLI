use serde::Deserialize; 
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenDetailsResponse {
    pub isValid: bool,
    pub expiry: DateTime<Utc>,
    pub userName: String,
}