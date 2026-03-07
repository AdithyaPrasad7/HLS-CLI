use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ValidateTokenResponse {
    pub data: bool,
    pub success: bool,
}