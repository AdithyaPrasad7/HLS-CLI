use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use std::sync::Arc;
use reqwest::{StatusCode};

use crate::client::{apiClient, error::Error};
use crate::model::{apiRequest, tokenResponse};
use crate::config::API_BASE_URL;


pub static API_CLIENT: Lazy<Arc<apiClient::ApiClient>> = Lazy::new(|| {
  Arc::new(apiClient::ApiClient::new(API_BASE_URL).unwrap())
});

pub async fn validateToken(token: &str) -> Result<tokenResponse::TokenDetailsResponse, Error> {

  let request = apiRequest::ApiRequest::new("/validate-token", "GET", "Validatng token...")
    .header("Content-Type", "application/json")
    .queryParam("token", token);

  let response: tokenResponse::TokenDetailsResponse = API_CLIENT.sendRequest(request).await?;
  if response.isValid {
    println!("Welcome {}!", response.userName);
    println!("Your Token is Valid till {}", response.expiry);
  } else {
    return Err(Error::BadRequest { message: "Token is invalid!".to_string() })
  };
  Ok(response)
}