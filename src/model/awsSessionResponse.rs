use serde::Deserialize; 
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionResponse {
    pub path: String,
    pub bucket: String,
    pub credentials: TemporaryCredentials
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemporaryCredentials {
    pub accessKey: String,
    pub secretKey: String,
    pub sessionToken: String,
    pub expiresIn: u32,
}