use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use std::sync::Arc;

use crate::client::{apiClient, error::Error};
use crate::model::apiRequest::ApiRequest;
use crate::config::API_BASE_URL;


pub static API_CLIENT: Lazy<Arc<apiClient::ApiClient>> = Lazy::new(|| {
    Arc::new(apiClient::ApiClient::new(API_BASE_URL).unwrap())
});