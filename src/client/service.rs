use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use std::sync::Arc;
use reqwest::{StatusCode};

use crate::client::{apiClient, error::Error};
use crate::model::{apiRequest, tokenResponse, awsSessionResponse, updateVideoStatus};
use crate::config::API_BASE_URL;
use crate::token::CONFIG;

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

pub async fn getAWSSession() -> Result<awsSessionResponse::SessionResponse, Error> {
  let token = match CONFIG.getToken() {
      Some(t) => t,
      None => return Err(Error::BadRequest { message: "Token not configured".to_string() })
  };
  let request = apiRequest::ApiRequest::new("/create-session", "GET", "Contacting server...")
    .header("Content-Type", "application/json")
    .queryParam("token", token);

  let response: awsSessionResponse::SessionResponse = API_CLIENT.sendRequest(request).await?;
  
  Ok(response)
}

pub async fn updateVideoDetails(path: String, success: bool) -> Result<bool, Error> {
  let token = match CONFIG.getToken() {
      Some(t) => t,
      None => return Err(Error::BadRequest { message: "Token not configured".to_string() })
  };
  let updateStatus = updateVideoStatus::UpdateVideoStatus {
    path: path,
    token: token,
    isValid: success,
  };
  let request = apiRequest::ApiRequest::new("/update-video-details", "PUT", "Contacting server...")
    .header("Content-Type", "application/json")
    .body(serde_json::to_value(updateStatus).unwrap());

  let response: bool = API_CLIENT.sendRequest(request).await?;
  
  Ok(response)
}