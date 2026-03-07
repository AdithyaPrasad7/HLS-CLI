use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use std::sync::Arc;
use reqwest::{StatusCode};

use crate::client::{apiClient, error::Error};
use crate::model::{apiRequest, tokenReponse};
use crate::config::API_BASE_URL;


pub static API_CLIENT: Lazy<Arc<apiClient::ApiClient>> = Lazy::new(|| {
  Arc::new(apiClient::ApiClient::new(API_BASE_URL).unwrap())
});

pub async fn validateToken(token: &str) -> Result<bool, Error> {

  let request = apiRequest::ApiRequest::new("/validate-token", "GET", "Validatng token...")
    .header("Content-Type", "application/json")
    .queryParam("token", token);

  let response: tokenReponse::ValidateTokenResponse = API_CLIENT.sendRequest(request).await?;
  if(response.data) {
    println!("Token is Valid");
  } else {
    println!("Token is invalid");
  };
  Ok(response.data)
}