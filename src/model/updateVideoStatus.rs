use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVideoStatus {
    pub path: String,
    pub token: String,
    pub isValid: bool
}