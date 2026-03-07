use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct ApiRequest {
  pub path: String,
  pub method: String,
  pub headers: HashMap<String, String>,
  pub queryParams: HashMap<String, String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub body: Option<serde_json::Value>,
  pub loaderMessage: String
}

impl ApiRequest {
  pub fn new(path: &str, method: &str, loaderMessage: &str) -> Self {
    Self {
      path: path.to_string(),
      method: method.to_string(),
      headers: HashMap::new(),
      queryParams: HashMap::new(),
      body: None,
      loaderMessage: loaderMessage.to_string()
    }
  }

  pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
    self.headers.insert(key.into(), value.into());
    self
  }

  pub fn body(mut self, body: serde_json::Value) -> Self {
    self.body = Some(body);
    self
  }

  pub fn queryParam(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
    self.queryParams.insert(key.into(), value.into());
    self
  }
}
